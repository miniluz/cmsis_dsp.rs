//! Filtering functions

use crate::check_length;
use fixed::types::I1F15;

/// Instance structure for the Q15 Biquad cascade filter
pub struct BiquadCascadeDf1InstQ15<'a> {
    num_stages: i8,
    state: &'a mut [I1F15],
    coeffs: &'a [I1F15],
    post_shift: i8,
}

impl<'a> BiquadCascadeDf1InstQ15<'a> {
    /// Create a new biquad cascade instance
    ///
    /// # Arguments
    ///
    /// * `num_stages` - Number of biquad stages
    /// * `state` - State buffer (must have length 4 * num_stages)
    /// * `coeffs` - Coefficient buffer (must have length 6 * num_stages in CMSIS format: {b0, 0, b1, b2, a1, a2} per stage)
    /// * `post_shift` - Post-shift value to apply to outputs
    ///
    /// # Panics
    ///
    /// Panics if state length != 4 * num_stages or coeffs length != 6 * num_stages
    pub fn new(
        num_stages: usize,
        state: &'a mut [I1F15],
        coeffs: &'a [I1F15],
        post_shift: i8,
    ) -> Self {
        assert_eq!(
            state.len(),
            4 * num_stages,
            "State length must be 4 * num_stages"
        );
        assert_eq!(
            coeffs.len(),
            6 * num_stages,
            "Coeffs length must be 6 * num_stages (CMSIS format)"
        );
        assert!(num_stages <= 127, "num_stages must fit in i8");

        Self {
            num_stages: num_stages as i8,
            state,
            coeffs,
            post_shift,
        }
    }
}

/// Q15 Biquad cascade Direct Form I filter processing
///
/// Processes input samples through a cascade of biquad filters.
///
/// # Panics
///
/// Panics if input and output do not have the same length.
pub fn biquad_cascade_df1_q15(
    inst: &mut BiquadCascadeDf1InstQ15,
    input: &[I1F15],
    output: &mut [I1F15],
) {
    let length: u32 = check_length((input.len(), output.len()));

    // Construct the C struct directly (don't call init, as that would reset state)
    let c_inst = cmsis_dsp_sys::arm_biquad_casd_df1_inst_q15 {
        numStages: inst.num_stages,
        pState: inst.state.as_mut_ptr() as *mut _,
        pCoeffs: inst.coeffs.as_ptr() as *const _,
        postShift: inst.post_shift,
    };

    unsafe {
        // Process the data
        cmsis_dsp_sys::arm_biquad_cascade_df1_q15(
            &c_inst,
            input.as_ptr() as *const _,
            output.as_mut_ptr() as *mut _,
            length,
        );
    }
}
