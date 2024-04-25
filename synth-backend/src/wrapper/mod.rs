use crate::oscillators::{MultiOscillator, Oscillator, WaveTableOscillator};
use crate::filters::{Filter, FilterParam, FilterType};
use crate::envelopes::Envelope;
use crate::vibrato::Vibrato;
use std::ops::Add;

#[derive(Clone, Debug)]
pub struct Synth {
    pub osc: MultiOscillator,
    pub sample_rate: u32,
    pub filter: Option<Filter>, // Make filter an optional field
    pub envelope: Option<Envelope>,
    pub am_lfo: Option<WaveTableOscillator>,
    pub vibrato: Option<Vibrato>,
}

impl Synth {
    pub fn new(osc: MultiOscillator, sample_rate: u32, filter: Option<Filter>, envelope: Option<Envelope>, am_lfo: Option<WaveTableOscillator>, vibrato: Option<Vibrato>) -> Self {
        Self {
            osc,
            sample_rate,
            filter,
            envelope,
            am_lfo,
            vibrato,
        }
    }

    pub fn get_sample(&mut self) -> f32 {
        // Call the get_sample method of MultiOscillator
        let sample = self.osc.get_sample();
        let mut output_sample = sample;

        // Check if filter exists
        if let Some(ref mut filter) = self.filter {
            // Apply the filter if it exists
            output_sample = filter.process(sample);
        }

        if let Some(ref mut envelope) = self.envelope {
            output_sample = output_sample * envelope.get_amplitude();
        }

        if let Some(ref mut am_lfo) = self.am_lfo {
            let a = am_lfo.get_sample();
            output_sample = output_sample * (1.0 + a) / 2.0;
        }

        if let Some(ref mut vibrato) = self.vibrato {
            output_sample = vibrato.process(output_sample);
        }

        // If filter is None, return the sample directly
        output_sample
    }

    pub fn set_oscillator(&mut self, index: usize, oscillator: Oscillator) {
        self.osc.set_oscillator(index, oscillator);
    }

    pub fn push(&mut self, oscillator: WaveTableOscillator) -> Result<(), String> {
        self.osc.push(oscillator)
    }

    pub fn global_set_frequency(&mut self, frequency: f32) -> Result<(), String> {
        self.osc.global_set_frequency(frequency)
    }

    pub fn num_sources(&self) -> usize {
        self.osc.num_sources()
    }

    pub fn set_filter(&mut self, filter: Option<FilterType>, freq_filter: f32, bandwidth_hz_filter: f32) {
        match filter {
            None => self.filter = None,
            Some(filter_type) => match self.filter {
                None => self.filter = Some(Filter::new(filter_type, self.sample_rate as f32, freq_filter, bandwidth_hz_filter)),
                Some(_) => self.filter.as_mut().unwrap().change_filter_type(filter_type)
            }
        }
    }

    pub fn set_filter_params(&mut self, filterparam: FilterParam, value: f32) {
        match self.filter {
            None => (),
            Some(_) => self.filter.as_mut().unwrap().set_param(filterparam, value)
        }
    }
}
