use crate::oscillators::{MultiOscillator, Oscillator, WaveTableOscillator};
use crate::filters::{Filter, FilterParam, FilterType};
use crate::envelopes::{Envelope, EnvelopeParam};
use crate::lfo::{LFOType, LFO};
use std::ops::Add;

const GAIN: f32 = 1.0;
const WIDTH: f32 = 0.010;

#[derive(Clone, Debug)]
pub struct Synth {
    pub osc: MultiOscillator,
    pub sample_rate: u32,
    pub filter: Option<Filter>, // Make filter an optional field
    pub envelope: Option<Envelope>,
    pub lfo: Option<LFO>,
    pub lfo_type: LFOType,
}

impl Synth {
    pub fn new(osc: MultiOscillator, sample_rate: u32, filter: Option<Filter>, envelope: Option<Envelope>, lfo: Option<LFO>, lfo_type: LFOType) -> Self {
        Self {
            osc,
            sample_rate,
            filter,
            envelope,
            lfo,
            lfo_type
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

        if let Some(ref mut lfo) = self.lfo {
            output_sample = lfo.process(output_sample);
        }

        // If filter is None, return the sample directly
        output_sample
    }

    pub fn set_gain(&mut self, index: usize, gain: f32) -> Result<(), String> {
        self.osc.set_gain(gain, index)
    }

    pub fn set_oscillator(&mut self, index: usize, oscillator: Oscillator) {
        self.osc.set_oscillator(index, oscillator);
    }

    pub fn set_detune_semitones(&mut self, index: usize, detune_semitones: i8) -> Result<(), String> {
        self.osc.set_detune_semitones(detune_semitones, index)
    }

    pub fn set_lfo_type(&mut self, lfo_type: LFOType) {
        match self.lfo {
            None => {
                let lfo_type = self.lfo_type.clone();
                self.lfo.as_mut().unwrap().set_type(lfo_type)
            },
            Some(_) => self.lfo.as_mut().unwrap().set_type(lfo_type)
        }
    }

    pub fn remove(&mut self, index: usize) -> WaveTableOscillator {
        self.osc.remove(index)
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

    pub fn set_envelope_params(&mut self, envelope_param: EnvelopeParam, value: f32) {
        match self.envelope {
            None => (),
            Some(_) => self.envelope.as_mut().unwrap().set_param(envelope_param, value)
        }
    }

    pub fn set_lfo_frequency(&mut self, frequency: f32) {
        match self.lfo {
            None => (),
            Some(_) => self.lfo.as_mut().unwrap().set_frequency(frequency)
        }
    }

    pub fn set_lfo_osc(&mut self, oscillator: Option<Oscillator>, frequency: f32, lfo_type: LFOType){
        match oscillator {
            None => self.lfo = None,
            Some(osc) => match self.lfo {
                None => self.lfo = Some(LFO::new(
                    lfo_type,
                    self.sample_rate as f32,
                    WaveTableOscillator::new(self.sample_rate, self.sample_rate as usize, osc, GAIN, frequency),
                    WIDTH
                )),
                Some(_) => self.lfo.as_mut().unwrap().set_oscillator(osc)
            }
        }
        
    }

    pub fn get_lfo_osc(&mut self) -> Option<Oscillator> {
        match self.lfo {
            None => None,
            Some(_) => Some(self.lfo.as_mut().unwrap().get_oscillator())
        }
    }
}
