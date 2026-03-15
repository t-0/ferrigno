-- models/waldorf/streichfett.lua
-- Waldorf Streichfett (Desktop String Synthesizer)
-- 128-voice polyphony, Strings section (registration morph, ensemble, crescendo,
-- release) + Solo section (bass/e-piano/clavi/synth/pluto tone morph, tremolo,
-- AD/ASR envelope). Effects: Animate, Phaser, Reverb. 12 presets (A1-C4).
-- SysEx: F0 3E 19 <dev> ... F7 (Waldorf ID 3Eh, device 19h).
-- No NRPN support.

local cc = {
    -- Standard
    { cc_number=1,   note=-1, name="Modulation" },
    { cc_number=64,  note=-1, name="Sustain" },
    -- Strings
    { cc_number=70,  note=-1, name="Strings Registration" },
    { cc_number=71,  note=-1, name="Strings Octave" },
    { cc_number=72,  note=-1, name="Strings Release" },
    { cc_number=73,  note=-1, name="Strings Crescendo" },
    { cc_number=74,  note=-1, name="Ensemble Type" },
    { cc_number=75,  note=-1, name="Ensemble On/Off" },
    -- Solo
    { cc_number=76,  note=-1, name="Solo Tone" },
    { cc_number=77,  note=-1, name="Solo Tremolo" },
    { cc_number=78,  note=-1, name="Solo Split Mode" },
    { cc_number=79,  note=-1, name="Solo Envelope Type" },
    { cc_number=80,  note=-1, name="Solo Attack" },
    { cc_number=81,  note=-1, name="Solo Decay/Release" },
    -- Mix
    { cc_number=82,  note=-1, name="Balance" },
    -- Effects
    { cc_number=91,  note=-1, name="FX Type" },
    { cc_number=92,  note=-1, name="Animate Depth" },
    { cc_number=93,  note=-1, name="Phaser Depth" },
    { cc_number=94,  note=-1, name="Reverb Mix" },
}

return {
    name = "Waldorf Streichfett",
    cc   = cc,
}
