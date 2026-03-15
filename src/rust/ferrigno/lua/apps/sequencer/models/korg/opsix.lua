-- models/korg/opsix.lua
-- Korg opsix (Altered FM Synthesizer)
-- 6-operator FM with 30+ operator types (FM, ring mod, filter, effects),
-- 40 algorithms, 3 EGs, 3 LFOs, 3 effect slots, motion sequencer, mod matrix.
-- No NRPN support; deep editing via MIDI 2.0 Property Exchange (MIDI-CI).
-- SysEx: F0 42 00 00 ... F7 (Korg mfr ID 42, family 61 01).

local cc = {
    -- Performance / standard
    { cc_number=1,   note=-1, name="Modulation" },
    { cc_number=5,   note=-1, name="Glide Time" },
    { cc_number=7,   note=-1, name="Volume" },
    { cc_number=10,  note=-1, name="Pan" },
    { cc_number=11,  note=-1, name="Expression" },
    { cc_number=64,  note=-1, name="Damper" },
    { cc_number=65,  note=-1, name="Glide Mode" },
    { cc_number=66,  note=-1, name="Sostenuto" },
    { cc_number=67,  note=-1, name="Soft" },
    -- Synth parameters (front panel)
    { cc_number=70,  note=-1, name="Algorithm" },
    { cc_number=73,  note=-1, name="Attack" },
    { cc_number=74,  note=-1, name="Cutoff" },
    { cc_number=71,  note=-1, name="Resonance" },
    { cc_number=79,  note=-1, name="Decay/Release" },
    { cc_number=81,  note=-1, name="FX1" },
    { cc_number=82,  note=-1, name="FX2" },
    { cc_number=83,  note=-1, name="FX3" },
    -- Operator levels
    { cc_number=102, note=-1, name="OP1 Level" },
    { cc_number=103, note=-1, name="OP2 Level" },
    { cc_number=104, note=-1, name="OP3 Level" },
    { cc_number=105, note=-1, name="OP4 Level" },
    { cc_number=106, note=-1, name="OP5 Level" },
    { cc_number=107, note=-1, name="OP6 Level" },
    -- Operator ratios
    { cc_number=108, note=-1, name="OP1 Ratio" },
    { cc_number=109, note=-1, name="OP2 Ratio" },
    { cc_number=110, note=-1, name="OP3 Ratio" },
    { cc_number=111, note=-1, name="OP4 Ratio" },
    { cc_number=112, note=-1, name="OP5 Ratio" },
    { cc_number=113, note=-1, name="OP6 Ratio" },
    -- Assignable (unipolar, virtual patch sources)
    { cc_number=12,  note=-1, name="Assign UNI 1" },
    { cc_number=13,  note=-1, name="Assign UNI 2" },
    { cc_number=14,  note=-1, name="Assign UNI 3" },
    { cc_number=15,  note=-1, name="Assign UNI 4" },
    { cc_number=16,  note=-1, name="Assign UNI 5" },
    -- Assignable (bipolar, virtual patch sources, center=64)
    { cc_number=17,  note=-1, name="Assign BIP 1" },
    { cc_number=18,  note=-1, name="Assign BIP 2" },
    { cc_number=19,  note=-1, name="Assign BIP 3" },
    { cc_number=20,  note=-1, name="Assign BIP 4" },
    { cc_number=21,  note=-1, name="Assign BIP 5" },
}

return {
    name = "Korg opsix",
    cc   = cc,
}
