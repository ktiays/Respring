use crate::vector_arithmetic::VectorArithmetic;

/// A representation of a spring's motion.
#[derive(Debug, Clone, Copy)]
pub struct Spring {
    pub angular_frequency: f64,
    pub decay_constant: f64,
    pub mass: f64,
}

impl Spring {
    pub fn new(angular_frequency: f64, decay_constant: f64, mass: f64) -> Self {
        Self {
            angular_frequency,
            decay_constant,
            mass,
        }
    }
}

impl Spring {
    #[inline]
    pub fn with_duration(duration: f64) -> Self {
        Self::with_duration_bounce(duration, 0.0)
    }

    /// Creates a spring with the specified duration and bounce.
    ///
    /// # Arguments
    ///
    /// * `duration` - Defines the pace of the spring. This is approximately
    ///   equal to the settling duration, but for springs with very large
    ///   bounce values, will be the duration of the period of oscillation
    ///   for the spring.
    /// * `bounce` - How bouncy the spring should be. A value of 0 indicates
    ///   no bounces (a critically damped spring), positive values indicate
    ///   increasing amounts of bounciness up to a maximum of 1.0
    ///   (corresponding to undamped oscillation), and negative values
    ///   indicate overdamped springs with a minimum value of -1.0.
    pub fn with_duration_bounce(duration: f64, bounce: f64) -> Self {
        let mut angular_velocity_factor: f64 = -std::f64::consts::TAU;
        let mut damping_ratio: f64 = f64::INFINITY;

        // Calculate damping ratio based on bounce parameter
        if bounce > -1.0 {
            damping_ratio = 1.0;

            if bounce < 0.0 {
                // Handle overdamped case (negative bounce)
                damping_ratio = 1.0 / (bounce + 1.0);
            } else if bounce != 0.0 {
                // Handle underdamped case (positive bounce)
                damping_ratio = 0.0;
                if bounce <= 1.0 {
                    damping_ratio = 1.0 - bounce;
                }
            }

            // Adjust angular velocity factor for underdamped case
            if damping_ratio <= 1.0 {
                angular_velocity_factor = std::f64::consts::TAU;
            }
        }

        // Calculate final spring parameters
        let angular_frequency =
            (1.0 - damping_ratio * damping_ratio).abs().sqrt() * angular_velocity_factor / duration;
        let decay_constant = damping_ratio * std::f64::consts::TAU / duration;
        let mass = 1.0;

        Self {
            angular_frequency,
            decay_constant,
            mass,
        }
    }

    /// The perceptual duration, which defines the pace of the spring.
    #[inline]
    pub fn duration(&self) -> f64 {
        let omega = self.angular_frequency;
        let decay = self.decay_constant;
        let absolute_omega = omega.abs();
        std::f64::consts::TAU / (decay * decay + omega * absolute_omega).sqrt()
    }

    /// How bouncy the spring is.
    ///
    /// A value of 0 indicates no bounces (a critically damped spring), positive values indicate
    /// increasing amounts of bounciness up to a maximum of 1.0 (corresponding to undamped oscillation),
    /// and negative values indicate overdamped springs with a minimum value of -1.0.
    pub fn bounce(&self) -> f64 {
        let half_decay = self.decay_constant / 2.0;
        let decay_squared = self.decay_constant * self.decay_constant;
        let frequency_squared = self.angular_frequency * self.angular_frequency;

        if self.angular_frequency >= 0.0 {
            let oscillation_period =
                -std::f64::consts::TAU / (frequency_squared + decay_squared).sqrt();
            (oscillation_period * half_decay) / std::f64::consts::PI + 1.0
        } else {
            let decay_period = std::f64::consts::TAU / (decay_squared - frequency_squared).sqrt();
            1.0 / ((decay_period * half_decay) / std::f64::consts::PI) - 1.0
        }
    }
}

impl Spring {
    /// Creates a spring with the specified mass, stiffness, and damping.
    ///
    /// # Arguments
    ///
    /// * `mass` - Specifies that property of the object attached to the end of
    ///   the spring.
    /// * `stiffness` - The corresponding spring coefficient.
    /// * `damping` - Defines how the spring's motion should be damped due to the
    ///   forces of friction.
    /// * `allow_over_damping` - A value of true specifies that over-damping
    ///   should be allowed when appropriate based on the other inputs, and a
    ///   value of false specifies that such cases should instead be treated as
    ///   critically damped.
    pub fn with_mass_stiffness_damping(
        mass: f64,
        stiffness: f64,
        damping: f64,
        allow_over_damping: bool,
    ) -> Self {
        let natural_frequency = (stiffness / mass).sqrt();
        let damping_ratio = damping / (2.0 * mass);

        let (angular_frequency, decay_constant) =
            if damping_ratio > natural_frequency && !allow_over_damping {
                (0.0, natural_frequency)
            } else {
                let oscillation = (stiffness / mass - damping_ratio * damping_ratio)
                    .abs()
                    .sqrt();
                let angular_freq = if damping_ratio > natural_frequency {
                    -oscillation
                } else {
                    oscillation
                };
                (angular_freq, damping_ratio)
            };

        Self {
            angular_frequency,
            decay_constant,
            mass,
        }
    }

    /// The spring stiffness coefficient.
    ///
    /// Increasing the stiffness reduces the number of oscillations and will
    /// reduce the settling duration. Decreasing the stiffness increases the the
    /// number of oscillations and will increase the settling duration.
    #[inline]
    pub fn stiffness(&self) -> f64 {
        self.mass
            * (self.angular_frequency * self.angular_frequency
                + self.decay_constant * self.decay_constant)
    }

    /// Defines how the spring's motion should be damped due to the forces of
    /// friction.
    ///
    /// Reducing this value reduces the energy loss with each oscillation: the
    /// spring will overshoot its destination. Increasing the value increases
    /// the energy loss with each duration: there will be fewer and smaller
    /// oscillations.
    #[inline]
    pub fn damping(&self) -> f64 {
        self.decay_constant * 2.0 * self.mass
    }
}

impl Spring {
    /// Creates a spring with the specified response and damping ratio.
    ///
    /// # Arguments
    ///
    /// * `response` - Defines the stiffness of the spring as an approximate
    ///   duration in seconds.
    /// * `damping_ratio` - Defines the amount of drag applied as a fraction the
    ///   amount needed to produce critical damping.
    pub fn with_response_damping_ratio(response: f64, damping_ratio: f64) -> Self {
        // Calculate angular frequency and decay based on whether system is overdamped.
        let is_overdamped = damping_ratio > 1.0;
        let tau_factor = if is_overdamped {
            -std::f64::consts::TAU
        } else {
            std::f64::consts::TAU
        };
        let ratio_squared = damping_ratio * damping_ratio;
        let damping_offset = (1.0 - ratio_squared).abs();

        // Calculate final spring parameters.
        let frequency_component = damping_offset.sqrt();
        let angular_frequency = (tau_factor * frequency_component) / response;
        let decay_constant = (std::f64::consts::TAU * damping_ratio) / response;

        Self {
            angular_frequency,
            decay_constant,
            mass: 1.0,
        }
    }

    /// The stiffness of the spring, defined as an approximate duration in seconds.
    #[inline]
    pub fn response(&self) -> f64 {
        let damping_squared = self.decay_constant * self.decay_constant;
        let response_term = self.angular_frequency * self.angular_frequency.abs();
        std::f64::consts::TAU / (damping_squared + response_term).sqrt()
    }

    /// The amount of drag applied, as a fraction of the amount needed to
    /// produce critical damping.
    ///
    /// When `damping_ratio` is 1, the spring will smoothly decelerate to its
    /// final position without oscillating. Damping ratios less than 1 will
    /// oscillate more and more before coming to a complete stop.
    #[inline]
    pub fn damping_ratio(&self) -> f64 {
        self.decay_constant * self.response() / std::f64::consts::TAU
    }
}

impl Spring {
    /// Creates a spring with the specified duration and damping ratio.
    ///
    /// # Arguments
    ///
    /// * `settling_duration` - The approximate time it will take for the spring to come to rest.
    /// * `damping_ratio` - The amount of drag applied as a fraction of the amount needed to produce critical damping.
    /// * `epsilon` - The threshold for how small all subsequent values need to be before the spring is considered to have settled.
    pub fn with_settling_duration_damping_ratio(
        settling_duration: f64,
        damping_ratio: f64,
        epsilon: f64,
    ) -> Self {
        let damping_ratio = damping_ratio.clamp(f64::EPSILON, 1.0);
        let duration = settling_duration.clamp(0.01, 10.0);

        let damping_squared_duration = damping_ratio * damping_ratio * duration;
        let natural_frequency = (1.0 - damping_ratio * damping_ratio).sqrt();
        let damping_frequency_ratio = damping_ratio / natural_frequency;
        let damped_time = duration * damping_ratio;

        let find_root = |initial_guess: f64,
                         max_iterations: i32,
                         response: &dyn Fn(f64) -> f64,
                         derivative: &dyn Fn(f64) -> f64,
                         result: &mut f64|
         -> bool {
            let mut current_value: f64 = initial_guess;
            let mut time_scale: f64 = 1.0 / duration;
            let mut remaining_iterations = max_iterations;

            let mut scaled_value = time_scale * current_value;
            let mut approximation = scaled_value;

            current_value = response(approximation);
            let next_value = approximation - current_value / derivative(approximation);
            approximation = next_value;

            if next_value.is_infinite() || next_value.is_nan() {
                *result = approximation;
                return false;
            }
            if remaining_iterations == 1 {
                *result = approximation;
                return true;
            }
            scaled_value = next_value - response(next_value) / derivative(approximation);
            approximation = scaled_value;
            if scaled_value.is_infinite() || scaled_value.is_nan() {
                *result = approximation;
                return false;
            }
            remaining_iterations -= 2;
            if remaining_iterations == 0 {
                *result = approximation;
                return true;
            }

            let mut difference = next_value - scaled_value;
            loop {
                current_value = scaled_value - response(scaled_value) / derivative(approximation);
                approximation = current_value;
                if current_value.is_infinite() || current_value.is_nan() {
                    *result = approximation;
                    return false;
                }

                time_scale = (current_value - scaled_value).abs();
                if time_scale <= epsilon {
                    *result = approximation;
                    return difference <= epsilon * 1e5;
                }
                difference = scaled_value - current_value;
                scaled_value = current_value;
                remaining_iterations -= 1;
                if remaining_iterations <= 0 {
                    break;
                }
            }
            *result = approximation;
            true
        };

        let damped_oscillation = |x: f64| -> f64 {
            epsilon - (damping_frequency_ratio * (-damped_time * x).exp()).abs()
        };

        let damped_response = |x: f64| -> f64 {
            let squared_x = x * x;
            let damping_term = squared_x * damping_squared_duration;
            damping_term / ((damped_time * x).exp() * squared_x * natural_frequency)
        };

        let critical_response = |x: f64| -> f64 {
            let threshold = if x < 0.0 { -epsilon } else { epsilon };
            let response = duration * x;
            (-response).exp() * (response + 1.0) - threshold
        };

        let critical_derivative =
            |x: f64| -> f64 { -duration * duration * x / (duration * x).exp() };

        let (response_function, derivative_function): (&dyn Fn(f64) -> f64, &dyn Fn(f64) -> f64) =
            if damping_ratio >= 1.0 {
                (&critical_response, &critical_derivative)
            } else {
                (&damped_oscillation, &damped_response)
            };

        let mut root_value: f64 = 0.0;
        if find_root(
            5.0,
            12,
            response_function,
            derivative_function,
            &mut root_value,
        ) {
            _ = find_root(
                1.0,
                20,
                response_function,
                derivative_function,
                &mut root_value,
            );
        }

        let mut omega = root_value;
        let omega_squared = omega * omega;
        omega = omega * 2.0 * damping_ratio;
        let half_omega = omega / 2.0;
        omega = (omega_squared - half_omega * half_omega).abs().sqrt();
        let decay: f64 = if root_value >= half_omega {
            half_omega
        } else {
            root_value
        };
        if root_value < half_omega {
            omega = 0.0;
        }

        Self {
            angular_frequency: omega,
            decay_constant: decay,
            mass: 1.0,
        }
    }
}

impl Spring {
    /// The estimated duration required for the spring system to be considered
    /// at rest.
    ///
    /// This uses a `target` of 1.0, an `initial_velocity` of 0, and an `epsilon`
    /// of 0.001.
    pub fn settling_duration(&self) -> f64 {
        self.settling_duration_with_velocity(1.0, 0.0, 0.001)
    }

    /// The estimated duration required for the spring system to be considered at rest.
    ///
    /// The epsilon value specifies the threshold for how small all subsequent
    /// values need to be before the spring is considered to have settled.
    pub fn settling_duration_with_velocity<V>(
        &self,
        target: V,
        initial_velocity: V,
        epsilon: f64,
    ) -> f64
    where
        V: VectorArithmetic,
    {
        if self.decay_constant == 0.0 {
            return f64::INFINITY;
        }

        if self.angular_frequency <= 0.0 {
            let mut best_time = -1.0;
            let mut time: f64 = 0.0;
            let mut best_distance: f64 = f64::INFINITY;

            for _ in 0..1024 {
                let current_value = self.value(target.clone(), initial_velocity.clone(), time);
                let diff = current_value - target.clone();
                let distance = diff.magnitude_squared().sqrt();
                if distance.is_nan() || distance.is_infinite() {
                    break;
                }

                if best_distance >= epsilon {
                    if distance < best_distance {
                        best_time = time;
                        best_distance = distance;
                    }
                } else if distance >= epsilon {
                    best_distance = f64::INFINITY;
                } else if time - best_time > 1.0 {
                    return best_time;
                }

                time += 0.1;
            }

            return 0.0;
        }

        let magnitude = (target.clone().scaled_by(self.decay_constant) - initial_velocity)
            .magnitude_squared()
            .sqrt()
            + target.magnitude_squared().sqrt();
        let settling_time = -(epsilon / magnitude).ln() / self.decay_constant;
        settling_time.max(0.0)
    }

    /// Calculates the value of the spring at a given time given a target amount of change.
    pub fn value<V>(&self, target: V, initial_velocity: V, time: f64) -> V
    where
        V: VectorArithmetic,
    {
        if self.angular_frequency > 0.0 {
            let angle = self.angular_frequency * time;
            let sin_val = angle.sin();
            let cos_val = angle.cos();

            let displacement = (target.clone().scaled_by(self.decay_constant) - initial_velocity)
                .scaled_by(sin_val / self.angular_frequency)
                + target.clone().scaled_by(cos_val);
            target.clone() - displacement.scaled_by((-self.decay_constant * time).exp())
        } else if self.angular_frequency < 0.0 {
            let negative_freq_minus_damping = -self.angular_frequency - self.decay_constant;
            let exp_term1 = (negative_freq_minus_damping * time).exp();
            let exp_term2 = ((self.angular_frequency - self.decay_constant) * time).exp();

            let damping_factor = (self.decay_constant - self.angular_frequency) * exp_term1
                + negative_freq_minus_damping * exp_term2;
            let scale_factor = damping_factor / (self.angular_frequency * 2.0) + 1.0;
            let velocity_factor = (exp_term1 - exp_term2) / (self.angular_frequency * 2.0);

            target.clone().scaled_by(scale_factor)
                - initial_velocity.clone().scaled_by(velocity_factor)
        } else {
            let displacement = target.clone()
                + (target.clone().scaled_by(self.decay_constant) - initial_velocity.clone())
                    .scaled_by(time);
            let damping_term = (-self.decay_constant * time).exp();
            target.clone() - displacement.scaled_by(damping_term)
        }
    }

    /// Calculates the velocity of the spring at a given time given a target amount of change.
    pub fn velocity<V>(&self, target: V, initial_velocity: V, time: f64) -> V
    where
        V: VectorArithmetic,
    {
        if self.angular_frequency > 0.0 {
            let damping_term = (-self.decay_constant * time).exp();
            let angle = self.angular_frequency * time;
            let sin_val = angle.sin();
            let cos_val = angle.cos();

            let target_term = target.clone().scaled_by(
                (self.angular_frequency * sin_val + self.decay_constant * cos_val) * damping_term,
            );
            let displacement_factor =
                (self.decay_constant * sin_val - self.angular_frequency * cos_val) * damping_term
                    / self.angular_frequency;
            let velocity_term = (target.clone().scaled_by(self.decay_constant)
                - initial_velocity.clone())
            .scaled_by(displacement_factor);
            velocity_term + target_term
        } else if self.angular_frequency < 0.0 {
            let negative_freq_minus_damping = -self.angular_frequency - self.decay_constant;
            let damping_minus_freq = self.angular_frequency - self.decay_constant;

            let exp_term1 = (negative_freq_minus_damping * time).exp();
            let exp_term2 = (damping_minus_freq * time).exp();

            let term1 = negative_freq_minus_damping * exp_term1;
            let term2 = damping_minus_freq * exp_term2;

            let scale_factor = ((self.decay_constant - self.angular_frequency) * term1
                + negative_freq_minus_damping * term2)
                / (self.angular_frequency * 2.0)
                + 1.0;
            let velocity_factor = (term1 - term2) / (self.angular_frequency * 2.0);

            target.clone().scaled_by(scale_factor)
                - initial_velocity.clone().scaled_by(velocity_factor)
        } else {
            let damping_term = (-self.decay_constant * time).exp();
            let time_factor = (self.decay_constant * time - 1.0) * damping_term;
            let velocity_delta =
                target.clone().scaled_by(self.decay_constant) - initial_velocity.clone();
            let damped_target = target.clone().scaled_by(self.decay_constant * damping_term);
            velocity_delta.scaled_by(time_factor) + damped_target
        }
    }

    /// Updates the current value and velocity of a spring.
    ///
    /// # Arguments
    ///
    /// * `value` - The current value of the spring.
    /// * `velocity` - The current velocity of the spring.
    /// * `target` - The target that `value` is moving towards.
    /// * `delta_time` - The amount of time that has passed since the spring was
    ///   at the position specified by `value`.
    pub fn update<V>(&self, value: &mut V, velocity: &mut V, target: V, delta_time: f64)
    where
        V: VectorArithmetic,
    {
        let delta = target - value.clone();
        let delta_velocity = self.velocity(delta.clone(), velocity.clone(), delta_time);
        let delta_value = self.value(delta, velocity.clone(), delta_time);
        *velocity = delta_velocity;
        *value += delta_value;
    }

    /// Calculates the force upon the spring given a current position, target, and velocity amount of change.
    ///
    /// This value is in units of the vector type per second squared.
    pub fn force<V>(&self, target: V, position: V, velocity: V) -> V
    where
        V: VectorArithmetic,
    {
        let damping_force = velocity
            .clone()
            .scaled_by((-self.decay_constant * 2.0) * self.mass);
        let delta = target - position;
        let spring_force = delta.scaled_by(
            (self.angular_frequency * self.angular_frequency
                + self.decay_constant * self.decay_constant)
                * self.mass,
        );
        spring_force + damping_force
    }
}

impl Spring {
    /// A smooth spring with a predefined duration and no bounce.
    #[inline]
    pub fn smooth() -> Self {
        Self::with_duration_bounce(0.5, 0.0)
    }

    /// A smooth spring with a predefined duration and no bounce that can be
    /// tuned.
    ///
    /// # Arguments
    ///
    /// * `duration` - The perceptual duration, which defines the pace of the
    ///   spring. This is approximately equal to the settling duration, but
    ///   for very bouncy springs, will be the duration of the period of
    ///   oscillation for the spring.
    /// * `extra_bounce` - How much additional bounce should be added to the base
    ///   bounce of 0.
    #[inline]
    pub fn smooth_with_duration(duration: f64, extra_bounce: f64) -> Self {
        Self::with_duration_bounce(duration, extra_bounce)
    }

    /// A spring with a predefined duration and small amount of bounce that
    /// feels more snappy.
    #[inline]
    pub fn snappy() -> Self {
        Self::with_duration_bounce(0.5, 0.15)
    }

    /// A spring with a predefined duration and small amount of bounce that
    /// feels more snappy and can be tuned.
    ///
    /// # Arguments
    ///
    /// * `duration` - The perceptual duration, which defines the pace of the
    ///   spring. This is approximately equal to the settling duration, but
    ///   for very bouncy springs, will be the duration of the period of
    ///   oscillation for the spring.
    /// * `extra_bounce` - How much additional bounciness should be added to the
    ///   base bounce of 0.15.
    #[inline]
    pub fn snappy_with_duration(duration: f64, extra_bounce: f64) -> Self {
        Self::with_duration_bounce(duration, 0.15 + extra_bounce)
    }

    /// A spring with a predefined duration and higher amount of bounce.
    #[inline]
    pub fn bouncy() -> Self {
        Self::with_duration_bounce(0.5, 0.3)
    }

    /// A spring with a predefined duration and higher amount of bounce that
    /// can be tuned.
    ///
    /// # Arguments
    ///
    /// * `duration` - The perceptual duration, which defines the pace of the
    ///   spring. This is approximately equal to the settling duration, but
    ///   for very bouncy springs, will be the duration of the period of
    ///   oscillation for the spring.
    /// * `extra_bounce` - How much additional bounce should be added to the base
    ///   bounce of 0.3.
    #[inline]
    pub fn bouncy_with_duration(duration: f64, extra_bounce: f64) -> Self {
        Self::with_duration_bounce(duration, 0.3 + extra_bounce)
    }
}
