use core::sync::atomic::Ordering;
use core::{sync::atomic::AtomicBool, fmt::Debug};


use embedded_hal::digital::v2::InputPin as EmInputPin;
use embedded_hal::digital::v2::OutputPin as EmOutputPin;

use lc3_traits::peripherals::gpio::{GpioPinArr, GpioMiscError};


pub enum OwnedOrMut<'a, T> {
    Owned(T),
    Mut(&'a mut T),
}

impl<'a, T> OwnedOrMut<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            OwnedOrMut::Owned(o) => o,
            OwnedOrMut::Mut(m) => m,
        }
    }
}

impl<'a, T> From<T> for OwnedOrMut<'a, T> {
    fn from(o: T) -> Self {
        Self::Owned(o)
    }
}

impl<'a, T> From<&'a mut T> for OwnedOrMut<'a, T> {
    fn from(borrowed: &'a mut T) -> Self {
        Self::Mut(borrowed)
    }
}

pub struct Gpio<
    'c,
    'i,
    G0: Interrupts + IoPin<Ctx = Ctx>,
    G1: Interrupts + IoPin<Ctx = Ctx>,
    G2: Interrupts + IoPin<Ctx = Ctx>,
    G3: Interrupts + IoPin<Ctx = Ctx>,
    G4: Interrupts + IoPin<Ctx = Ctx>,
    G5: Interrupts + IoPin<Ctx = Ctx>,
    G6: Interrupts + IoPin<Ctx = Ctx>,
    G7: Interrupts + IoPin<Ctx = Ctx>,
    Ctx = (),
> {
    g0: Pin<G0>,
    g1: Pin<G1>,
    g2: Pin<G2>,
    g3: Pin<G3>,
    g4: Pin<G4>,
    g5: Pin<G5>,
    g6: Pin<G6>,
    g7: Pin<G7>,
    interrupt_flags: &'i GpioPinArr<AtomicBool>,
    ctx: OwnedOrMut<'c, Ctx>,
}

impl<'c, 'i, A, B, C, D, E, F, G, H, CC> Gpio<'c, 'i, A, B, C, D, E, F, G, H, CC>
where
    A: Interrupts + IoPin<Ctx = CC>,
    B: Interrupts + IoPin<Ctx = CC>,
    C: Interrupts + IoPin<Ctx = CC>,
    D: Interrupts + IoPin<Ctx = CC>,
    E: Interrupts + IoPin<Ctx = CC>,
    F: Interrupts + IoPin<Ctx = CC>,
    G: Interrupts + IoPin<Ctx = CC>,
    H: Interrupts + IoPin<Ctx = CC>,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new_with_context(
        g0: A::Disabled,
        g1: B::Disabled,
        g2: C::Disabled,
        g3: D::Disabled,
        g4: E::Disabled,
        g5: F::Disabled,
        g6: G::Disabled,
        g7: H::Disabled,
        interrupt_flags: &'i GpioPinArr<AtomicBool>,
        ctx: impl Into<OwnedOrMut<'c, CC>>,
    ) -> Self {
        use Pin::Disabled as D;
        Self {
            g0: D(g0),
            g1: D(g1),
            g2: D(g2),
            g3: D(g3),
            g4: D(g4),
            g5: D(g5),
            g6: D(g6),
            g7: D(g7),
            interrupt_flags,
            ctx: ctx.into(),
        }
    }
}

impl<'c, 'i, A, B, C, D, E, F, G, H> Gpio<'c, 'i, A, B, C, D, E, F, G, H, ()>
where
    A: Interrupts + IoPin<Ctx = ()>,
    B: Interrupts + IoPin<Ctx = ()>,
    C: Interrupts + IoPin<Ctx = ()>,
    D: Interrupts + IoPin<Ctx = ()>,
    E: Interrupts + IoPin<Ctx = ()>,
    F: Interrupts + IoPin<Ctx = ()>,
    G: Interrupts + IoPin<Ctx = ()>,
    H: Interrupts + IoPin<Ctx = ()>,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        g0: A::Disabled,
        g1: B::Disabled,
        g2: C::Disabled,
        g3: D::Disabled,
        g4: E::Disabled,
        g5: F::Disabled,
        g6: G::Disabled,
        g7: H::Disabled,
        interrupt_flags: &'i GpioPinArr<AtomicBool>
    ) -> Self {
        Self::new_with_context(g0, g1, g2, g3, g4, g5, g6, g7, interrupt_flags, ())
    }
}

#[derive(Debug)]
enum Pin<P: Interrupts + IoPin> {
    Disabled(P::Disabled),
    Input(P::Input),
    Interrupt(P::Input),
    Output(P::Output),

    Transitioning,
}

// TODO: implement: From<T: Display> for GpioMiscError; log T during conversion.


// TODO: need to discuss what behavior we want
//
// For now:
// Once a pin is set to output but before a write the pin is? 0? unknown? implementation defined?
//   -> Setting to 0 on transition to output
// Write to the register in input mode? Ignored
//   -> faults! raises an error!
// Read from the register in output mode? 0s? or do we cache the last written value?
//   -> stateful, returns the last written value
//      * consideration: haven't surveyed the embedded_hal impls to see if this is
//        truly burdensome but this technically necessitates an additional trait impl:
//        StatefulOutputPin. we may be trading board support for this functionality;
//        that would be an argument for *not* being stateful (i.e. fault on read of an
//        output pin)

/*
    &mut self
        Pat(ref mut binding)
            Pat(binding) <- mem::replace(&mut self, x : T)
                -> T
                    &mut self <- Pat::Dual(T into _)

    &mut self
        Pat(ref mut binding)
            Pat(binding) <- mem::replace(&mut self, x : T)
                -> T (as s)
                    -> &mut T // transform, nested call
                        Pat(ref mut binding)
                            Pat(binding) <- mem::replace(&mut s, x: T)
                                -> T (as y)
                                    &mut self <- Pat::Dual(y into _)
                    &mut self <- Pat::Dual(s, modified)
*/

/// $self: &mut T
/// $pat should accept either ref mut or value
/// $e should return value
macro_rules! transform {
    (($self:ident, $new_state:ident) => {
        $(
            $($variants:ident)|+ $(as $value_binding:ident)? => $next:tt in $e:expr,
        )+

        $(@else => $else:expr $(,)?)?
    }) => {
        // The idea is that this is correct by construction; it can see the match
        // pattern so it will use the right pattern after `mem::replac`ing.
        match ($self, $new_state) {
            $(

                ($(__ref_binding @ Self::$variants(_))|+, lc3_gp::GpioState::$next) => {
                    #[allow(unused_mut)]
                    {
                        let __self: &mut Pin<_> = __ref_binding;
                        let mut __tmp: Pin<_> = core::mem::replace(__self, Self::Transitioning);

                        // Expose as `$self` if no value binding is given:
                        $(#[cfg(all(disable, $value_binding))])?
                        let mut $self = __tmp;

                        // If a value binding was given, refine `$self`
                        // further and expose that:
                        transform!(
                            (@cond_bind: __tmp)
                            ($($variants)+)
                            ($($value_binding)?)
                        );

                        // Expand the expression:
                        #[warn(unused_mut)]
                        let res = {
                            $e
                        };

                        #[allow(unreachable_code, unused)]
                        { *__self = Self::$next(res); }

                        Ok(())
                    }
                }
            )+

            (Self::Transitioning, _) => unreachable!(),

            $(_ => $else)?
        }
    };

    ((@cond_bind: $rhs:ident) ($($variants:ident)+) ($value_binding:ident)) => {
        let mut $value_binding = if let $(Self::$variants(x))|+ = $rhs {
            x
        } else {
            unreachable!()
        };
    };
    ((@cond_bind: $rhs:ident) ($($variants:ident)+) ()) => { };
}

impl<P: Interrupts + IoPin> Pin<P> {
    fn set_state(&mut self, new_state: lc3_gp::GpioState, ctx: &mut P::Ctx) -> Result<(), lc3_gp::GpioMiscError> {
        let errf = <GpioMiscError>::from_source::<P::Error>;
        // let outp_errf = GpioMiscError::from_source::<<P::Output as embedded_hal::digital::v2::OutputPin>::Error>;
        // let outp_errf = GpioMiscError::from_source::<
        //     <
        //         P::Output
        //         as ImplsDebug<
        //             <P::Output as embedded_hal::digital::v2::OutputPin>::Error,
        //             Error = <P::Output as embedded_hal::digital::v2::OutputPin>::Error,
        //         >
        //     >::Inner
        // >;

        let this = self;
        transform! {
            (this, new_state) => {
                // No transition (assuming this optimizes away...)
                Disabled  as dis => Disabled  in dis,
                Output    as out => Output    in out,
                Input     as inp => Input     in inp,
                Interrupt as inp => Interrupt in inp,

                // To disabled:
                Output    as out => Disabled in P::output_to_disabled(out, ctx).map_err(errf)?,
                Input     as inp => Disabled in P::input_to_disabled(inp, ctx).map_err(errf)?,
                Interrupt as inp => Disabled in {
                    P::disable_interrupts(&mut inp, ctx).map_err(errf)?;
                    P::input_to_disabled(inp, ctx).map_err(errf)?
                },

                // To output:
                Disabled | Input | Interrupt => Output in {
                    let borrow = &mut this;
                    transform! ((borrow, new_state) => {
                        Disabled   as dis  => Output in P::disabled_to_output(dis, ctx).map_err(errf)?,
                        Input      as inp  => Output in P::input_to_output(inp, ctx).map_err(errf)?,
                        Interrupt  as inp  => Output in {
                            P::disable_interrupts(&mut inp, ctx).map_err(errf)?;
                            P::input_to_output(inp, ctx).map_err(errf)?
                        },
                        @else => unreachable!(),
                    })?;

                    // Pins are always to be low when switched to output mode.
                    if let Self::Output(mut out) = this {
                        EmOutputPin::set_low(&mut out).map_err(GpioMiscError::from_source)?;
                        out
                    } else { unreachable!() }
                },

                // To input:
                Disabled  as dis => Input in P::disabled_to_input(dis, ctx).map_err(errf)?,
                Output    as out => Input in P::output_to_input(out, ctx).map_err(errf)?,
                Interrupt as inp => Input in {
                    P::disable_interrupts(&mut inp, ctx).map_err(errf)?;
                    inp
                },

                // To interrupt:
                Disabled | Input | Output => Interrupt in {
                    let borrow = &mut this;
                    transform! ((borrow, new_state) => {
                        Disabled as dis => Interrupt in P::disabled_to_input(dis, ctx).map_err(errf)?,
                        Input    as inp => Interrupt in inp,
                        Output   as out => Interrupt in P::output_to_input(out, ctx).map_err(errf)?,
                        @else => unreachable!(),
                    })?;

                    // Now that we've switched to Input mode, enabled interrupts"
                    if let Self::Interrupt(mut inp) = this {
                        P::enable_interrupts(&mut inp, ctx).map_err(errf)?;
                        inp
                    } else { unreachable!() }
                },
            }
        }
    }

    fn get_state(&self) -> lc3_gp::GpioState {
        use Pin::*;
        use lc3_gp::GpioState as G;
        match self {
            Disabled(_) => G::Disabled,
            Input(_) => G::Input,
            Interrupt(_) => G::Interrupt,
            Output(_) => G::Output,
            Transitioning => unreachable!(),
        }
    }

    fn read(&self) -> Result<bool, lc3_gp::GpioReadError> {
        use Pin::*;
        use lc3_gp::GpioReadError as Err;
        match self {
            Input(inp) | Interrupt(inp) => {
                EmInputPin::is_high(inp).map_err(GpioMiscError::from_source).map_err(Into::into)
            },
            Disabled(_) => Err(Err::IsDisabled),
            Output(_) => Err(Err::IsInOutputMode),
            Transitioning => unreachable!(), // TODO: unchecked!
        }
    }

    fn write(&mut self, bit: bool) -> Result<(), lc3_gp::GpioWriteError> {
        use Pin::*;
        use lc3_gp::GpioWriteError as Err;
        match self {
            Output(out) => {
                EmOutputPin::set_state(out, bit.into()).map_err(GpioMiscError::from_source).map_err(Into::into)
            },
            Disabled(_) => Err(Err::IsDisabled),
            Input(_) => Err(Err::IsInInputMode),
            Interrupt(_) => Err(Err::IsInInterruptMode),
            Transitioning => unreachable!(), // TODO: unchecked!
        }
    }
}

/* macro_rules! transform {
    (($self:ident, $new_state:ident) => {
        $(
            $($p:pat)|+ => $next:tt in $e:expr,
        )+

        $(@else => $else:expr $(,)?)?
    }) => {
        // The idea is that this is correct by construction; it can see the match
        // pattern so it will use the right pattern after `mem::replac`ing.
        match ($self, $new_state) {
            $(

                #[allow(unused)]
                ($($p)|+, lc3_gp::GpioState::$next) => {
                    let __self: &mut _ = $self;
                    #[allow(unused_mut)]
                    let mut __tmp: Pin<_> = core::mem::replace(__self, Self::Transitioning);

                    #[warn(unused)]
                    {
                        if let $($p)|+ = __tmp {
                            let res = $e;

                            // #[allow(unreachable_code, unused)]
                            { *__self = Self::$next(res); }

                            Ok(())
                        } else {
                            unreachable!()
                        }
                    }
                }
            )+

            (Self::Transitioning, _) => unreachable!(),

            $(_ => $else)?
        }
    };
}
 */


// impl<P: Interrupts + IoPin> Pin<P> {
//     fn set_state(&mut self, new_state: lc3_gp::GpioState, ctx: &mut P::Ctx) -> Result<(), lc3_gp::GpioMiscError> {
//         // use lc3_gp::GpioState::*;
//         use Pin::*;

//         // match (self, new_state) {
//         //     (__s, __n) => match (__s, new_state) {
//         //         (Disabled(dis), lc3_gp::GpioState::Disabled) => {
//         //             #[allow(unused_mut)]
//         //             let mut __tmp = core::mem::replace(__s, Self::Transitioning);
//         //             if let Disabled(dis) = __tmp {
//         //                 let res = dis;
//         //                 *__s = Self::Disabled(res);
//         //                 Ok(())
//         //             } else {
//         //                 panic!()
//         //             }
//         //         }
//         //         (Output(out), lc3_gp::GpioState::Output) => {
//         //             #[allow(unused_mut)]
//         //             let mut __tmp = core::mem::replace(__s, Self::Transitioning);
//         //             if let Output(out) = __tmp {
//         //                 let res = out;
//         //                 *__s = Self::Output(res);
//         //                 Ok(())
//         //             } else {
//         //                 panic!()
//         //             }
//         //         }
//         //         (Input(inp), lc3_gp::GpioState::Input) => {
//         //             #[allow(unused_mut)]
//         //             let mut __tmp = core::mem::replace(__s, Self::Transitioning);
//         //             if let Input(inp) = __tmp {
//         //                 let res = inp;
//         //                 *__s = Self::Input(res);
//         //                 Ok(())
//         //             } else {
//         //                 panic!()
//         //             }
//         //         }
//         //         (Interrupt(inp), lc3_gp::GpioState::Interrupt) => {
//         //             #[allow(unused_mut)]
//         //             let mut __tmp = core::mem::replace(__s, Self::Transitioning);
//         //             if let Interrupt(inp) = __tmp {
//         //                 let res = inp;
//         //                 *__s = Self::Interrupt(res);
//         //                 Ok(())
//         //             } else {
//         //                 panic!()
//         //             }
//         //         }
//         //         (Output(out), lc3_gp::GpioState::Disabled) => {
//         //             #[allow(unused_mut)]
//         //             let mut __tmp = core::mem::replace(__s, Self::Transitioning);
//         //             if let Output(out) = __tmp {
//         //                 let res = P::output_to_disabled(out, ctx).map_err(|_| GpioMiscError)?;
//         //                 *__s = Self::Disabled(res);
//         //                 Ok(())
//         //             } else {
//         //                 panic!()
//         //             }
//         //         }
//         //         (Input(inp), lc3_gp::GpioState::Disabled) => {
//         //             #[allow(unused_mut)]
//         //             let mut __tmp = core::mem::replace(__s, Self::Transitioning);
//         //             if let Input(inp) = __tmp {
//         //                 let res = P::input_to_disabled(inp, ctx).map_err(|_| GpioMiscError)?;
//         //                 *__s = Self::Disabled(res);
//         //                 Ok(())
//         //             } else {
//         //                 panic!()
//         //             }
//         //         }
//         //         (Interrupt(inp), lc3_gp::GpioState::Disabled) => {
//         //             #[allow(unused_mut)]
//         //             let mut __tmp = core::mem::replace(__s, Self::Transitioning);
//         //             if let Interrupt(mut inp) = __tmp {
//         //                 let res = {
//         //                     P::disable_interrupts(&mut inp, ctx).map_err(|_| GpioMiscError)?;
//         //                     P::input_to_disabled(inp, ctx).map_err(|_| GpioMiscError)?
//         //                 };
//         //                 *__s = Self::Disabled(res);
//         //                 Ok(())
//         //             } else {
//         //                 panic!()
//         //             }
//         //         }
//         //         (
//         //             ref mut s @ Disabled(_) | ref mut s @ Input(_) | ref mut s @ Interrupt(_),
//         //             lc3_gp::GpioState::Output,
//         //         ) => {
//         //             #[allow(unused_mut)]
//         //             let mut __tmp = core::mem::replace(__s, Self::Transitioning);
//         //             if let s @ Disabled(_)
//         //             | s @ Input(_)
//         //             | s @ Interrupt(_) = __tmp
//         //             {
//         //                 let res = {
//         //                     let _: Result<(), GpioMiscError> = match (s, new_state) {
//         //                         (ref mut __s, __n) => match (__s, new_state) {
//         //                             (Disabled(dis), lc3_gp::GpioState::Output) => {
//         //                                 #[allow(unused_mut)]
//         //                                 let mut __tmp =
//         //                                     core::mem::replace(__s, Self::Transitioning);
//         //                                 if let Disabled(dis) = __tmp {
//         //                                     let res = P::disabled_to_output(dis, ctx)
//         //                                         .map_err(|_| GpioMiscError)?;
//         //                                     *__s = Self::Output(res);
//         //                                     Ok(())
//         //                                 } else {
//         //                                     panic!(
//         //                                         "internal error: entered unreachable code",
//         //                                     )
//         //                                 }
//         //                             }
//         //                             (Input(inp), lc3_gp::GpioState::Output) => {
//         //                                 #[allow(unused_mut)]
//         //                                 let mut __tmp =
//         //                                     core::mem::replace(__s, Self::Transitioning);
//         //                                 if let Input(inp) = __tmp {
//         //                                     let res = P::input_to_output(inp, ctx)
//         //                                         .map_err(|_| GpioMiscError)?;
//         //                                     *__s = Self::Output(res);
//         //                                     Ok(())
//         //                                 } else {
//         //                                     panic!(
//         //                                         "internal error: entered unreachable code",
//         //                                     )
//         //                                 }
//         //                             }
//         //                             (Interrupt(ref mut inp), lc3_gp::GpioState::Output) => {
//         //                                 #[allow(unused_mut)]
//         //                                 let mut __tmp =
//         //                                     core::mem::replace(__s, Self::Transitioning);
//         //                                 if let Interrupt(mut inp) = __tmp {
//         //                                     let res = {
//         //                                         P::disable_interrupts(&mut inp, ctx)
//         //                                             .map_err(|_| GpioMiscError)?;
//         //                                         P::input_to_output(inp, ctx)
//         //                                             .map_err(|_| GpioMiscError)?
//         //                                     };
//         //                                     *__s = Self::Output(res);
//         //                                     Ok(())
//         //                                 } else {
//         //                                     panic!(
//         //                                         "internal error: entered unreachable code",
//         //                                     )
//         //                                 }
//         //                             }
//         //                             (_, lc3_gp::GpioState::Output) => {
//         //                                 #[allow(unused_mut)]
//         //                                 let mut __tmp =
//         //                                     core::mem::replace(__s, Self::Transitioning);
//         //                                 if let _ = __tmp {
//         //                                     let res = panic!(
//         //                                         "internal error: entered unreachable code",
//         //                                     );
//         //                                     *__s = Self::Output(res);
//         //                                     Ok(())
//         //                                 } else {
//         //                                     panic!(
//         //                                         "internal error: entered unreachable code",
//         //                                     )
//         //                                 }
//         //                             }
//         //                             _ => panic!(),
//         //                             (Self::Transitioning, _) => panic!(
//         //                                 "internal error: entered unreachable code",
//         //                             ),
//         //                         },
//         //                     };
//         //                     if let Output(ref mut out) = s {
//         //                         OutputPin::set_low(out).map_err(|_| GpioMiscError)?;
//         //                         out
//         //                     } else {
//         //                         panic!()
//         //                     }
//         //                 };
//         //                 *__s = Self::Output(res);
//         //                 Ok(())
//         //             } else {
//         //                 panic!()
//         //             }
//         //         }
//         //         (Disabled(dis), lc3_gp::GpioState::Input) => {
//         //             #[allow(unused_mut)]
//         //             let mut __tmp = core::mem::replace(__s, Self::Transitioning);
//         //             if let Disabled(dis) = __tmp {
//         //                 let res = P::disabled_to_input(dis, ctx).map_err(|_| GpioMiscError)?;
//         //                 *__s = Self::Input(res);
//         //                 Ok(())
//         //             } else {
//         //                 panic!()
//         //             }
//         //         }
//         //         (Output(out), lc3_gp::GpioState::Input) => {
//         //             #[allow(unused_mut)]
//         //             let mut __tmp = core::mem::replace(__s, Self::Transitioning);
//         //             if let Output(out) = __tmp {
//         //                 let res = P::output_to_input(out, ctx).map_err(|_| GpioMiscError)?;
//         //                 *__s = Self::Input(res);
//         //                 Ok(())
//         //             } else {
//         //                 panic!()
//         //             }
//         //         }
//         //         (Interrupt(inp), lc3_gp::GpioState::Input) => {
//         //             #[allow(unused_mut)]
//         //             let mut __tmp = core::mem::replace(__s, Self::Transitioning);
//         //             if let Interrupt(mut inp) = __tmp {
//         //                 let res = {
//         //                     P::disable_interrupts(&mut inp, ctx).map_err(|_| GpioMiscError)?;
//         //                     inp
//         //                 };
//         //                 *__s = Self::Input(res);
//         //                 Ok(())
//         //             } else {
//         //                 panic!()
//         //             }
//         //         }
//         //         _ => todo!(),
//         //         (Self::Transitioning, _) => {
//         //             panic!()
//         //         }
//         //     },
//         // }


// /*         &mut self
//             Pat(ref mut binding)
//                 Pat(binding) <- mem::replace(&mut self, x : T)
//                     -> T
//                         &mut self <- Pat::Dual(T into _)

//         &mut self
//             Pat(ref mut binding)
//                 Pat(binding) <- mem::replace(&mut self, x : T)
//                     -> T (as s)
//                         -> &mut T // transform, nested call
//                             Pat(ref mut binding)
//                                 Pat(binding) <- mem::replace(&mut s, x: T)
//                                     -> T (as y)
//                                         &mut self <- Pat::Dual(y into _)
//                         &mut self <- Pat::Dual(s, modified) */

//         // transform! {
//         //     (self, new_state) => {
//         //         // No transition (assuming this optimizes away...)
//         //         Disabled(dis)  => Disabled  in dis,
//         //         Output(out)    => Output    in out,
//         //         Input(inp)     => Input     in inp,
//         //         Interrupt(inp) => Interrupt in inp,

//         //         // To disabled:
//         //         Output(out)    => Disabled in P::output_to_disabled(out, ctx).map_err(|_| GpioMiscError)?,
//         //         Input(inp)     => Disabled in P::input_to_disabled(inp, ctx).map_err(|_| GpioMiscError)?,
//         //         Interrupt(inp) => Disabled in {
//         //             let mut inp = inp;
//         //             P::disable_interrupts(&mut inp, ctx).map_err(|_| GpioMiscError)?;
//         //             P::input_to_disabled(inp, ctx).map_err(|_| GpioMiscError)?
//         //         },

//         //         // To output:
//         //         mut s @ Disabled(_) | mut s @ Input(_) | mut s @ Interrupt(_) => Output in {
//         //             let s_borrow: &mut Pin<_> = &mut s;

//         //             transform! ((s_borrow, new_state) => {
//         //                 Disabled(dis)  => Output in P::disabled_to_output(dis, ctx).map_err(|_| GpioMiscError)?,
//         //                 Input(inp)     => Output in P::input_to_output(inp, ctx).map_err(|_| GpioMiscError)?,
//         //                 Interrupt(inp) => Output in {
//         //                     let mut inp = inp;
//         //                     P::disable_interrupts(&mut inp, ctx).map_err(|_| GpioMiscError)?;
//         //                     P::input_to_output(inp, ctx).map_err(|_| GpioMiscError)?
//         //                 },
//         //                 @else => unreachable!(),
//         //             })?;

//         //             // Pins are always to be low when switched to output mode.
//         //             if let Output(mut out) = s {
//         //                 OutputPin::set_low(&mut out).map_err(|_| GpioMiscError)?;
//         //                 out
//         //             } else { unreachable!() }
//         //         },

//         //         // To input:
//         //         Disabled(dis)  => Input in P::disabled_to_input(dis, ctx).map_err(|_| GpioMiscError)?,
//         //         Output(out)    => Input in P::output_to_input(out, ctx).map_err(|_| GpioMiscError)?,
//         //         Interrupt(inp) => Input in {
//         //             let mut inp = inp;
//         //             P::disable_interrupts(&mut inp, ctx).map_err(|_| GpioMiscError)?;
//         //             inp
//         //         },

//         //         // To interrupt:
//         //         _ => Interrupt in unreachable!(),

//         //     }
//         // }


//         transform! {
//             (self, new_state) => {
//                 // No transition (assuming this optimizes away...)
//                 Disabled(dis)  => Disabled  in dis,

//                 // // To output:
//                 // mut s @ Disabled(_) | mut s @ Input(_) | mut s @ Interrupt(_) => Output in {
//                 //     let s_borrow: &mut Pin<_> = &mut s;

//                 //     transform! ((s_borrow, new_state) => {
//                 //         Disabled(dis)  => Output in P::disabled_to_output(dis, ctx).map_err(|_| GpioMiscError)?,
//                 //         @else => unreachable!(),
//                 //     })?;

//                 //     // Pins are always to be low when switched to output mode.
//                 //     if let Output(out) = s {
//                 //         out
//                 //     } else { unreachable!() }
//                 // },

//                 @else => unreachable!(),
//             }
//         }

//         //         // (Input(inp) | Interrupt(inp), Disabled) => {

//         //         // }
//         //     }
//         // }

//         // match (self, new_state) {
//         //     // No transition.
//         //     (Self::Disabled(_), Disabled) | (Self::Input(_), Input) | (Self::Interrupt(_), Interrupt) | (Self::Output(_), Output) => {
//         //         Ok(())
//         //     },

//         //     // To disabled:
//         //     (Self::Input(inp) | Self::Interrupt(inp), Disabled) => {
//         //         if let Self::Interrupt(ref mut inp) = self {
//         //             P::disable_interrupts(inp, ctx).map_err(|_| GpioMiscError)?;
//         //         }

//         //         *self = Self::Disabled(P::input_to_disabled(*inp, ctx).map_err(|_| GpioMiscError)?);
//         //         Ok(())
//         //     },
//         //     (Self::Output(inp), Disabled) => {
//         //         *self = Self::Disabled(P::output_to_disabled(*inp, ctx).map_err(|_| GpioMiscError)?);
//         //         Ok(())
//         //     },

//         // }
//     }
// }

// trait IoPin where
//     Self::Pin<Self::InpTypeState>: embedded_hal::digital::v2::InputPin,
//     Self::Pin<Self::OutTypeState>: embedded_hal::digital::v2::OutputPin + embedded_hal::digital::v2::StatefulOutputPin,
// {
//     type InpTypeState;
//     type OutTypeState;
//     type OffTypeState;

//     type Pin<TypeState>;

//     type Ctx;

//     fn into_input<S>(p: Self::Pin<S>, ctx: &mut Self::Ctx) -> Self::Pin<Self::InpTypeState>;
//     fn into_output<S>(p: Self::Pin<S>, ctx: &mut Self::Ctx) -> Self::Pin<Self::OutTypeState>;
//     fn into_disabled<S>(p: Self::Pin<S>, ctx: &mut Self::Ctx) -> Self::Pin<Self::OffTypeState>;
// }

// TODO: add compat blanket impl for: https://github.com/rust-embedded/embedded-hal/issues/29
// where Ctx = () and Off = Inp
//
// will need an impl of `set_interrupt` though...
//
// actually we should just offer a macro to do the impl.

// pub trait EmbeddedHalInputWithDebugErrorType: embedded_hal::digital::v2::InputPin {
//     type Error: Debug;
// }
// impl<I: embedded_hal::digital::v2::InputPin> EmbeddedHalInputWithDebugErrorType for I
// where I::Error: Debug,
// {
//     type Error = I::Error;
// }

// pub trait EmbeddedHalOutputWithDebugErrorType: embedded_hal::digital::v2::OutputPin {
//     type Error: Debug;
// }
// impl<O: embedded_hal::digital::v2::OutputPin> EmbeddedHalOutputWithDebugErrorType for O
// where O::Error: Debug,
// {
//     type Error = ();
// }

pub trait EmbeddedHalV2InputPinWithErrorImplingDebug: EmInputPin<Error = Self::BoundedErrorType> {
    type BoundedErrorType: Debug;
}
impl<I: EmInputPin> EmbeddedHalV2InputPinWithErrorImplingDebug for I
where
    <I as EmInputPin>::Error: Debug,
{
    type BoundedErrorType = I::Error;
}
pub trait EmbeddedHalV2OutputPinWithErrorImplingDebug: EmOutputPin<Error = Self::BoundedErrorType> {
    type BoundedErrorType: Debug;
}
impl<O: EmOutputPin> EmbeddedHalV2OutputPinWithErrorImplingDebug for O
where
    <O as EmOutputPin>::Error: Debug,
{
    type BoundedErrorType = O::Error;
}

pub trait IoPin
where
    // This is unfortunate; we want these to be requirements of implementing the trait
    // not WF requirements for users of this trait...
    // <Self::Input as embedded_hal::digital::v2::InputPin>::Error: Debug,
    // <Self::Output as embedded_hal::digital::v2::OutputPin>::Error: Debug,
{
    type Ctx;

    type Disabled;
    type Input: EmInputPin + EmbeddedHalV2InputPinWithErrorImplingDebug;
        // where
            // <Self::Input as embedded_hal::digital::v2::InputPin>::Error: Debug,
        // + EmbeddedHalInputWithDebugErrorType<Error = <Self::Input as embedded_hal::digital::v2::InputPin>::Error>
    type Output: EmOutputPin + EmbeddedHalV2OutputPinWithErrorImplingDebug;
    //  + ImplsDebug<
    //     <Self::Output as embedded_hal::digital::v2::OutputPin>::Error,
    //     Inner = <Self::Output as embedded_hal::digital::v2::OutputPin>::Error,
    // >
    //     // + EmbeddedHalOutputWithDebugErrorType<Error = <Self::Output as embedded_hal::digital::v2::OutputPin>::Error>
    // ;

    type Error: Debug;
    // TODO: can't require Into... gpio error types, bc of coherence; we don't
    // expect users to be able to modify their HAL crates (they can't impl our
    // trait or std's trait for a foreign type)

    fn disabled_to_input(d: Self::Disabled, ctx: &mut Self::Ctx) -> Result<Self::Input, Self::Error>;
    fn disabled_to_output(d: Self::Disabled, ctx: &mut Self::Ctx) -> Result<Self::Output, Self::Error>;

    fn input_to_disabled(i: Self::Input, ctx: &mut Self::Ctx) -> Result<Self::Disabled, Self::Error>;
    fn input_to_output(i: Self::Input, ctx: &mut Self::Ctx) -> Result<Self::Output, Self::Error>;

    fn output_to_disabled(o: Self::Output, ctx: &mut Self::Ctx) -> Result<Self::Disabled, Self::Error>;
    fn output_to_input(o: Self::Output, ctx: &mut Self::Ctx) -> Result<Self::Input, Self::Error>;
}

// pub trait IoPinDebugMarker {}
// impl<I: IoPin> IoPinDebugMarker for I
// where
//     <I::Input as embedded_hal::digital::v2::InputPin>::Error: Debug,
//     <I::Output as embedded_hal::digital::v2::OutputPin>::Error: Debug,
// {}

pub trait Interrupts: IoPin {
    fn enable_interrupts(i: &mut Self::Input, ctx: &mut Self::Ctx) -> Result<(), Self::Error>;
    fn disable_interrupts(i: &mut Self::Input, ctx: &mut Self::Ctx) -> Result<(), Self::Error>;
}
// ^ being split off means you can _just_ implement ^ in the case where
// you're using an embedded_hal::IoPin impl

// TODO: need to make a Spec for board implemenntors
//
// i.e. rising edge interrupts
// tristate disable
// etc.
//
// reccomendations, not requirements

pub use crate::io_pins_with_typestate;
#[macro_export]
macro_rules! io_pins_with_typestate {
    (
        $(#![$($mod_meta:tt)*])*

        for pins {$(
            $(#[$($meta:tt)*])*
            $pin_struct:ident as $alias:ident,
        )*} as $gpio_alias:ident;

        type Ctx = $ctx:ty;
        type Error = $err:ty;

        type Disabled = $disabled:ty;
        type Input = $input:ty;
        type Output = $output:ty;

        => disabled = |$to_d:pat, $c_d:pat| $to_disabled:expr
        => input    = |$to_i:pat, $c_i:pat| $to_input:expr
        => output   = |$to_o:pat, $c_o:pat| $to_output:expr

        // $(
        => +interrupts = |$int_en:pat,  $c_ie:pat| $int_enabled:expr
        => -interrupts = |$int_dis:pat, $c_di:pat| $int_disabled:expr
        // )?
    ) => {
        $(#[$($mod_meta)*])*
        pub mod io_pins {
            #[allow(unused)]
            use super::*;
            #[derive(Debug)]
            struct Private;

            $(
                // Not constructable!
                $(#[$($meta)*])*
                #[derive(Debug)]
                pub struct $alias(Private);

                impl $crate::generic_gpio::IoPin for $alias {
                    type Ctx = $ctx;

                    type Disabled = $pin_struct<$disabled>;
                    type Input = $pin_struct<$input>;
                    type Output = $pin_struct<$output>;

                    type Error = $err;

                    fn input_to_disabled(
                        $to_d: Self::Input, $c_d: &mut Self::Ctx,
                    ) -> Result<Self::Disabled, Self::Error> { $to_disabled }
                    fn output_to_disabled(
                        $to_d: Self::Output, $c_d: &mut Self::Ctx,
                    ) -> Result<Self::Disabled, Self::Error> { $to_disabled }

                    fn disabled_to_input(
                        $to_i: Self::Disabled, $c_i: &mut Self::Ctx,
                    ) -> Result<Self::Input, Self::Error> { $to_input }
                    fn output_to_input(
                        $to_i: Self::Output, $c_i: &mut Self::Ctx,
                    ) -> Result<Self::Input, Self::Error> { $to_input }

                    fn disabled_to_output(
                        $to_o: Self::Disabled, $c_o: &mut Self::Ctx,
                    ) -> Result<Self::Output, Self::Error> { $to_output }
                    fn input_to_output(
                        $to_o: Self::Input, $c_o: &mut Self::Ctx,
                    ) -> Result<Self::Output, Self::Error> { $to_output }
                }

                // TODO: split off into separate macro:
                // $(
                    impl $crate::generic_gpio::Interrupts for $alias {
                        fn enable_interrupts(
                            $int_en: &mut Self::Input, $c_ie: &mut Self::Ctx,
                        ) -> Result<(), Self::Error> {
                            $int_enabled
                        }
                        fn disable_interrupts(
                            $int_dis: &mut Self::Input, $c_di: &mut Self::Ctx,
                        ) -> Result<(), Self::Error> {
                            $int_disabled
                        }
                    }
                // )?
            )*
        }

        pub use io_pins::{
            $($alias),*
        };
        pub type $gpio_alias<'c, 'i> = $crate::generic_gpio::Gpio<
            'c, 'i,
            $($alias,)*
            $ctx,
        >;
    };
}

// use tm4c123x_hal::gpio::{
//     self as gp,
//     PushPull,
//     PullDown,
//     gpiof::{PF1, PF2, PF4},
//     gpiob::{PB3, PB4, PB5, PB6, PB7},
// };

// io_pins_with_typestate! {
//     #![allow(clippy::unit_arg)]
//     //! TODO: module doc comment!

//     for pins {
//         /// yo
//         PF1 as G0,
//         /// yo
//         PF2 as G1,
//         /// yo
//         PF4 as G2,
//         /// yo
//         PB3 as G3,
//         /// yo
//         PB4 as G4,
//         /// yo
//         PB5 as G5,
//         /// yo
//         PB6 as G6,
//         /// yo
//         PB7 as G7,
//     }:

//     type Ctx = ();
//     type Error = Infallible;

//     type Disabled = gp::Tristate;
//     type Input = gp::Input<PullDown>;
//     type Output = gp::Output<PushPull>;

//     => disabled = |x, ()| Ok(x.into_tri_state())
//     => input    = |x, ()| Ok(x.into_pull_down_input())
//     => output   = |x, ()| Ok(x.into_push_pull_output())

//     => +interrupts = |inp, ()| Ok(inp.set_interrupt_mode(gp::InterruptMode::EdgeRising))
//     => -interrupts = |inp, ()| Ok(inp.set_interrupt_mode(gp::InterruptMode::Disabled))
// }


// impl<'c, 'i, A, B, C, D, E, F, G, H, CC> Default for Gpio<'c, 'i, A, B, C, D, E, F, G, H, CC>
// where
//     A: Interrupts + IoPin<Ctx = CC>,
//     B: Interrupts + IoPin<Ctx = CC>,
//     C: Interrupts + IoPin<Ctx = CC>,
//     D: Interrupts + IoPin<Ctx = CC>,
//     E: Interrupts + IoPin<Ctx = CC>,
//     F: Interrupts + IoPin<Ctx = CC>,
//     G: Interrupts + IoPin<Ctx = CC>,
//     H: Interrupts + IoPin<Ctx = CC>,
// {
//     fn default() -> Self {
//         panic!("remove this super trait requirement! doesn't make sense for embedded");
//     }
// }

macro_rules! pin_proxy {
    ($self:ident[$gp_pin:ident] as $binding:pat => $e:expr) => { {
        use lc3_gp::GpioPin::*;
        pin_proxy!(@arms: $gp_pin
            (G0 to g0),
            (G1 to g1),
            (G2 to g2),
            (G3 to g3),
            (G4 to g4),
            (G5 to g5),
            (G6 to g6),
            (G7 to g7),
            in $self:
                ($binding => $e)
        )
    }};

    (@arms: $gp_pin:ident $(($gp:ident to $field:ident)),* $(,)? in $self:ident: ($binding:pat => $e:expr)) => {
        match $gp_pin {
            $(
                $gp => {
                    let $binding = $self.$field;
                    $e
                }
            )*
        }
    };
}

use lc3_traits::peripherals::gpio::{self as lc3_gp};

// #[allow(clippy::toplevel_ref_arg)]
// impl<'c, 'i, A, B, C, D, E, F, G, H, CC> Default for Gpio<'c, 'i, A, B, C, D, E, F, G, H, CC>
// where
//     A: Interrupts + IoPin<Ctx = CC>,
//     B: Interrupts + IoPin<Ctx = CC>,
//     C: Interrupts + IoPin<Ctx = CC>,
//     D: Interrupts + IoPin<Ctx = CC>,
//     E: Interrupts + IoPin<Ctx = CC>,
//     F: Interrupts + IoPin<Ctx = CC>,
//     G: Interrupts + IoPin<Ctx = CC>,
//     H: Interrupts + IoPin<Ctx = CC>,
// {
//     fn default() -> Self {
//         panic!("remove this requirement!");
//     }
// }

#[allow(clippy::toplevel_ref_arg)]
impl<'c, 'i, A, B, C, D, E, F, G, H, CC> lc3_gp::Gpio<'i> for Gpio<'c, 'i, A, B, C, D, E, F, G, H, CC>
where
    A: Interrupts + IoPin<Ctx = CC>,
    B: Interrupts + IoPin<Ctx = CC>,
    C: Interrupts + IoPin<Ctx = CC>,
    D: Interrupts + IoPin<Ctx = CC>,
    E: Interrupts + IoPin<Ctx = CC>,
    F: Interrupts + IoPin<Ctx = CC>,
    G: Interrupts + IoPin<Ctx = CC>,
    H: Interrupts + IoPin<Ctx = CC>,
{
    fn set_state(&mut self, pin: lc3_gp::GpioPin, state: lc3_gp::GpioState) -> Result<(), GpioMiscError> {
        pin_proxy!(
            self[pin] as ref mut p => {
                p.set_state(state, self.ctx.as_mut())
            }
        )
    }

    fn get_state(&self, pin: lc3_gp::GpioPin) -> lc3_gp::GpioState {
        pin_proxy!(self[pin] as ref p => p.get_state())
    }

    // TODO: change ReadError to be more specific? not sure
    //
    // definitely also allow for "Other" errors...
    //
    // also don't let us return the pin, it doesn't make any sense to..
    fn read(&self, pin: lc3_gp::GpioPin) -> Result<bool, lc3_gp::GpioReadError> {
        pin_proxy!(self[pin] as ref p => p.read())
    }


    fn write(&mut self, pin: lc3_gp::GpioPin, bit: bool) -> Result<(), lc3_gp::GpioWriteError> {
        pin_proxy!(self[pin] as ref mut p => p.write(bit))
    }


    fn register_interrupt_flags(&mut self, _flags: & 'i GpioPinArr<AtomicBool>) {
        /* todo: remove this! */
    }


    fn interrupt_occurred(&self, pin: lc3_gp::GpioPin) -> bool {
        debug_assert!(matches!(self.get_state(pin), lc3_gp::GpioState::Interrupt));

        self.interrupt_flags[pin].load(Ordering::SeqCst)
    }


    fn reset_interrupt_flag(&mut self, pin: lc3_gp::GpioPin) {
        self.interrupt_flags[pin].store(false, Ordering::SeqCst);
    }

}
