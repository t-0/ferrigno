-- models/sequential/prophet08.lua
-- DSI / Sequential Prophet '08 (8-voice analog polysynth)
-- 2 oscillators, Curtis filter (2/4 pole), 3 envelopes, 4 LFOs, gated sequencer.
-- Dual-layer architecture: Layer A NRPNs 0-199, Layer B = Layer A + 200.
-- DSI synths prefer NRPN for parameter control; CCs cover a subset.
-- SysEx: F0 01 23 <cmd> ... F7 (DSI mfr ID 01, device 23h).

-- ── CC names ────────────────────────────────────────────────────────────────────
-- These work when Global > MIDI Param Send/Receive includes CC.

local cc = {
    -- Standard controllers (always active)
    { cc_number=1,   note=-1, name="Mod Wheel" },
    { cc_number=2,   note=-1, name="Breath Controller" },
    { cc_number=4,   note=-1, name="Foot Controller" },
    { cc_number=7,   note=-1, name="Volume" },
    { cc_number=11,  note=-1, name="Expression" },
    { cc_number=32,  note=-1, name="Bank Select" },
    { cc_number=64,  note=-1, name="Sustain" },
    { cc_number=74,  note=-1, name="Brightness" },
    -- Oscillator (CC mode)
    { cc_number=20,  note=-1, name="Osc 1 Frequency" },
    { cc_number=21,  note=-1, name="Osc 1 Fine Tune" },
    { cc_number=22,  note=-1, name="Osc 1 Shape" },
    { cc_number=23,  note=-1, name="Glide 1" },
    { cc_number=24,  note=-1, name="Osc 2 Frequency" },
    { cc_number=25,  note=-1, name="Osc 2 Fine Tune" },
    { cc_number=26,  note=-1, name="Osc 2 Shape" },
    { cc_number=27,  note=-1, name="Glide 2" },
    { cc_number=28,  note=-1, name="Osc Mix" },
    { cc_number=29,  note=-1, name="Noise Level" },
    -- Filter (CC mode)
    { cc_number=102, note=-1, name="Filter Frequency" },
    { cc_number=103, note=-1, name="Resonance" },
    { cc_number=104, note=-1, name="Filter Key Amount" },
    { cc_number=105, note=-1, name="Filter Audio Mod" },
    { cc_number=106, note=-1, name="Filter Env Amount" },
    { cc_number=107, note=-1, name="Filter Env Velocity" },
    { cc_number=108, note=-1, name="Filter Env Delay" },
    { cc_number=109, note=-1, name="Filter Env Attack" },
    { cc_number=110, note=-1, name="Filter Env Decay" },
    { cc_number=111, note=-1, name="Filter Env Sustain" },
    { cc_number=112, note=-1, name="Filter Env Release" },
    -- Amp (CC mode)
    { cc_number=113, note=-1, name="VCA Level" },
    { cc_number=114, note=-1, name="Pan Spread" },
    { cc_number=115, note=-1, name="Amp Env Amount" },
    { cc_number=116, note=-1, name="Amp Env Velocity" },
    { cc_number=117, note=-1, name="Amp Env Delay" },
    { cc_number=118, note=-1, name="Amp Env Attack" },
    { cc_number=119, note=-1, name="Amp Env Decay" },
    { cc_number=75,  note=-1, name="Amp Env Sustain" },
    { cc_number=76,  note=-1, name="Amp Env Release" },
    -- Envelope 3 (CC mode)
    { cc_number=85,  note=-1, name="Env 3 Destination" },
    { cc_number=86,  note=-1, name="Env 3 Amount" },
    { cc_number=87,  note=-1, name="Env 3 Velocity" },
    { cc_number=88,  note=-1, name="Env 3 Delay" },
    { cc_number=89,  note=-1, name="Env 3 Attack" },
    { cc_number=90,  note=-1, name="Env 3 Decay" },
    { cc_number=77,  note=-1, name="Env 3 Sustain" },
    { cc_number=78,  note=-1, name="Env 3 Release" },
    -- Transport (CC mode)
    { cc_number=14,  note=-1, name="BPM" },
    { cc_number=15,  note=-1, name="Clock Divide" },
}

-- ── NRPN names (Layer A) ────────────────────────────────────────────────────────
-- For Layer B, add 200 to each NRPN number.

local nrpn = {
    -- Oscillators
    { nrpn_number=0,   note=-1, name="Osc 1 Frequency" },
    { nrpn_number=1,   note=-1, name="Osc 1 Fine Tune" },
    { nrpn_number=2,   note=-1, name="Osc 1 Shape" },
    { nrpn_number=3,   note=-1, name="Osc 1 Glide" },
    { nrpn_number=4,   note=-1, name="Osc 1 Keyboard" },
    { nrpn_number=5,   note=-1, name="Osc 2 Frequency" },
    { nrpn_number=6,   note=-1, name="Osc 2 Fine Tune" },
    { nrpn_number=7,   note=-1, name="Osc 2 Shape" },
    { nrpn_number=8,   note=-1, name="Osc 2 Glide" },
    { nrpn_number=9,   note=-1, name="Osc 2 Keyboard" },
    { nrpn_number=10,  note=-1, name="Sync" },
    { nrpn_number=11,  note=-1, name="Glide Mode" },
    { nrpn_number=12,  note=-1, name="Osc Slop" },
    { nrpn_number=13,  note=-1, name="Osc Mix" },
    { nrpn_number=14,  note=-1, name="Noise Level" },
    -- Filter
    { nrpn_number=15,  note=-1, name="Filter Frequency" },
    { nrpn_number=16,  note=-1, name="Resonance" },
    { nrpn_number=17,  note=-1, name="Filter Key Amount" },
    { nrpn_number=18,  note=-1, name="Filter Audio Mod" },
    { nrpn_number=19,  note=-1, name="Filter Poles" },
    { nrpn_number=20,  note=-1, name="Filter Env Amount" },
    { nrpn_number=21,  note=-1, name="Filter Env Velocity" },
    { nrpn_number=22,  note=-1, name="Filter Env Delay" },
    { nrpn_number=23,  note=-1, name="Filter Env Attack" },
    { nrpn_number=24,  note=-1, name="Filter Env Decay" },
    { nrpn_number=25,  note=-1, name="Filter Env Sustain" },
    { nrpn_number=26,  note=-1, name="Filter Env Release" },
    -- Amplifier
    { nrpn_number=27,  note=-1, name="VCA Initial Level" },
    { nrpn_number=28,  note=-1, name="Pan Spread" },
    { nrpn_number=29,  note=-1, name="Voice Volume" },
    { nrpn_number=30,  note=-1, name="VCA Env Amount" },
    { nrpn_number=31,  note=-1, name="VCA Env Velocity" },
    { nrpn_number=32,  note=-1, name="VCA Env Delay" },
    { nrpn_number=33,  note=-1, name="VCA Env Attack" },
    { nrpn_number=34,  note=-1, name="VCA Env Decay" },
    { nrpn_number=35,  note=-1, name="VCA Env Sustain" },
    { nrpn_number=36,  note=-1, name="VCA Env Release" },
    -- LFO 1
    { nrpn_number=37,  note=-1, name="LFO 1 Frequency" },
    { nrpn_number=38,  note=-1, name="LFO 1 Shape" },
    { nrpn_number=39,  note=-1, name="LFO 1 Amount" },
    { nrpn_number=40,  note=-1, name="LFO 1 Destination" },
    { nrpn_number=41,  note=-1, name="LFO 1 Key Sync" },
    -- LFO 2
    { nrpn_number=42,  note=-1, name="LFO 2 Frequency" },
    { nrpn_number=43,  note=-1, name="LFO 2 Shape" },
    { nrpn_number=44,  note=-1, name="LFO 2 Amount" },
    { nrpn_number=45,  note=-1, name="LFO 2 Destination" },
    { nrpn_number=46,  note=-1, name="LFO 2 Key Sync" },
    -- LFO 3
    { nrpn_number=47,  note=-1, name="LFO 3 Frequency" },
    { nrpn_number=48,  note=-1, name="LFO 3 Shape" },
    { nrpn_number=49,  note=-1, name="LFO 3 Amount" },
    { nrpn_number=50,  note=-1, name="LFO 3 Destination" },
    { nrpn_number=51,  note=-1, name="LFO 3 Key Sync" },
    -- LFO 4
    { nrpn_number=52,  note=-1, name="LFO 4 Frequency" },
    { nrpn_number=53,  note=-1, name="LFO 4 Shape" },
    { nrpn_number=54,  note=-1, name="LFO 4 Amount" },
    { nrpn_number=55,  note=-1, name="LFO 4 Destination" },
    { nrpn_number=56,  note=-1, name="LFO 4 Key Sync" },
    -- Envelope 3
    { nrpn_number=57,  note=-1, name="Env 3 Destination" },
    { nrpn_number=58,  note=-1, name="Env 3 Amount" },
    { nrpn_number=59,  note=-1, name="Env 3 Velocity" },
    { nrpn_number=60,  note=-1, name="Env 3 Delay" },
    { nrpn_number=61,  note=-1, name="Env 3 Attack" },
    { nrpn_number=62,  note=-1, name="Env 3 Decay" },
    { nrpn_number=63,  note=-1, name="Env 3 Sustain" },
    { nrpn_number=64,  note=-1, name="Env 3 Release" },
    -- Mod routing 1-4
    { nrpn_number=65,  note=-1, name="Mod 1 Source" },
    { nrpn_number=66,  note=-1, name="Mod 1 Amount" },
    { nrpn_number=67,  note=-1, name="Mod 1 Destination" },
    { nrpn_number=68,  note=-1, name="Mod 2 Source" },
    { nrpn_number=69,  note=-1, name="Mod 2 Amount" },
    { nrpn_number=70,  note=-1, name="Mod 2 Destination" },
    { nrpn_number=71,  note=-1, name="Mod 3 Source" },
    { nrpn_number=72,  note=-1, name="Mod 3 Amount" },
    { nrpn_number=73,  note=-1, name="Mod 3 Destination" },
    { nrpn_number=74,  note=-1, name="Mod 4 Source" },
    { nrpn_number=75,  note=-1, name="Mod 4 Amount" },
    { nrpn_number=76,  note=-1, name="Mod 4 Destination" },
    -- Sequencer destinations
    { nrpn_number=77,  note=-1, name="Seq 1 Destination" },
    { nrpn_number=78,  note=-1, name="Seq 2 Destination" },
    { nrpn_number=79,  note=-1, name="Seq 3 Destination" },
    { nrpn_number=80,  note=-1, name="Seq 4 Destination" },
    -- Controller routing
    { nrpn_number=81,  note=-1, name="Mod Wheel Amount" },
    { nrpn_number=82,  note=-1, name="Mod Wheel Destination" },
    { nrpn_number=83,  note=-1, name="Pressure Amount" },
    { nrpn_number=84,  note=-1, name="Pressure Destination" },
    { nrpn_number=85,  note=-1, name="Breath Amount" },
    { nrpn_number=86,  note=-1, name="Breath Destination" },
    { nrpn_number=87,  note=-1, name="Velocity Amount" },
    { nrpn_number=88,  note=-1, name="Velocity Destination" },
    { nrpn_number=89,  note=-1, name="Foot Ctrl Amount" },
    { nrpn_number=90,  note=-1, name="Foot Ctrl Destination" },
    -- Performance
    { nrpn_number=91,  note=-1, name="BPM" },
    { nrpn_number=92,  note=-1, name="Clock Divide" },
    { nrpn_number=93,  note=-1, name="Pitch Bend Range" },
    { nrpn_number=94,  note=-1, name="Seq Trigger Mode" },
    { nrpn_number=95,  note=-1, name="Key Mode" },
    { nrpn_number=96,  note=-1, name="Unison Mode" },
    { nrpn_number=97,  note=-1, name="Arp Mode" },
    { nrpn_number=98,  note=-1, name="Env 3 Repeat" },
    { nrpn_number=99,  note=-1, name="Unison On/Off" },
    { nrpn_number=100, note=-1, name="Arp On/Off" },
    { nrpn_number=101, note=-1, name="Sequencer On/Off" },
}

return {
    name = "Prophet '08",
    cc   = cc,
    nrpn = nrpn,
}
