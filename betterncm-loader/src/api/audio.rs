use betterncm_macro::*;
use tracing::*;

#[betterncm_native_api(name = "audio.getFFTData")]
#[instrument]
pub fn get_fft_data() -> anyhow::Result<Vec<f32>> {
    Ok(crate::audio::get_fft_data().to_owned())
}

#[betterncm_native_api(name = "audio.acquireFFTData")]
#[instrument]
pub fn acquire_fft_data() {
    crate::audio::add_fft_counter();
}

#[betterncm_native_api(name = "audio.releaseFFTData")]
#[instrument]
pub fn release_fft_data() {
    crate::audio::sub_fft_counter();
}
