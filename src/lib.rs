#[macro_export]
macro_rules! state_machine {
	($name: ident; $($event: ident),+$(,)*; $($n: ident: $t: ty),*$(,)*) => {
		pub mod $name {
			pub enum Trans {
				None,
				Pop,
				Push(Box<dyn State>),
				Switch(Box<dyn State>),
				Quit,
			}

			impl Trans {
				pub fn none() -> Self { Trans::None }
				pub fn pop() -> Self { Trans::Pop }
				pub fn quit() -> Self { Trans::Quit }
				pub fn push(state: impl State) -> Self { Trans::Push(Box::new(state)) }
				pub fn switch(state: impl State) -> Self { Trans::Switch(Box::new(state)) }
			}

			pub trait State: Send + Sync + 'static {
				fn on_start(&mut self, $($n: $t,)*) {}
				fn on_stop(&mut self, $($n: $t,)*) {}
				fn on_pause(&mut self, $($n: $t,)*) {}
				fn on_resume(&mut self, $($n: $t,)*) {}

				fn update(&mut self, $($n: $t,)*) -> Trans { Trans::None }
				fn trigger(&mut self, event: Event, $($n: $t,)*) -> Trans { Trans::None }
			}

			pub enum Event {
				$($event,)+
			}

			pub struct StateMachine {
				pub running: bool,
				pub state_stack: Vec<Box<dyn State>>,
			}

			impl StateMachine {
				pub fn new(initial_state: impl State) -> Self {
					Self {
						running: false,
						state_stack: vec![Box::new(initial_state)],
					}
				}

				pub fn start(&mut self, $($n: $t,)*) {
					if !self.running {
						let state = self.state_stack.last_mut().unwrap();
						state.on_start($($n,)*);
						self.running = true;
					}
				}

				pub fn update(&mut self, $($n: $t,)*) {
					let trans = if let Some(state) = self.state_stack.last_mut() {
						state.update($($n,)*)
					} else {
						Trans::None
					};
					self.transition(trans, $($n,)*);
				}

				fn transition(&mut self, request: Trans, $($n: $t,)*) {
					if self.running {
						match request {
							Trans::None => (),
							Trans::Pop => self.pop($($n,)*),
							Trans::Push(state) => self.push(state, $($n,)*),
							Trans::Switch(state) => self.switch(state, $($n,)*),
							Trans::Quit => self.stop($($n,)*),
						}
					}
				}

				pub fn switch(&mut self, mut state: Box<dyn State>, $($n: $t,)*) {
					if self.running {
						if let Some(mut state) = self.state_stack.pop() {
							state.on_stop($($n,)*)
						}

						state.on_start($($n,)*);
						self.state_stack.push(state);
					}
				}

				pub fn push(&mut self, mut state: Box<dyn State>, $($n: $t,)*) {
					if self.running {
						if let Some(state) = self.state_stack.last_mut() {
							state.on_pause($($n,)*);
						}

						state.on_start($($n,)*);
						self.state_stack.push(state);
					}
				}

				pub fn pop(&mut self, $($n: $t,)*) {
					if self.running {
						if let Some(mut state) = self.state_stack.pop() {
							state.on_stop($($n,)*);
						}

						if let Some(state) = self.state_stack.last_mut() {
							state.on_resume($($n,)*);
						} else {
							self.running = false;
						}
					}
				}

				pub fn stop(&mut self, $($n: $t,)*) {
					if self.running {
						while let Some(mut state) = self.state_stack.pop() {
							state.on_stop($($n,)*);
						}

						self.running = false;
					}
				}

				pub fn is_state<T: State>(&self) -> bool {
					std::any::Any::is::<T>(&self.state_stack[0])
				}

				pub fn trigger(&mut self, event: Event, $($n: $t,)*) {
					if self.running {
						let trans = if let Some(state) = self.state_stack.last_mut() {
							state.trigger(event, $($n,)*)
						} else {
							Trans::None
						};
						self.transition(trans, $($n,)*);
					}
				}
			}
		}
	};
}
