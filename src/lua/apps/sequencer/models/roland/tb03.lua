-- models/roland/tb03.lua
-- Roland TB-03 (Boutique Bass Line, TB-303 recreation)
-- Monophonic bass synthesizer with accent, slide, overdrive, and delay.

-- ── CC names ────────────────────────────────────────────────────────────────

local cc = {
    { cc_number=12,  note=-1, name="Env Mod" },
    { cc_number=16,  note=-1, name="Accent" },
    { cc_number=17,  note=-1, name="Overdrive" },
    { cc_number=18,  note=-1, name="Delay Time" },
    { cc_number=19,  note=-1, name="Delay Feedback" },
    { cc_number=71,  note=-1, name="Resonance" },
    { cc_number=74,  note=-1, name="Cutoff" },
    { cc_number=75,  note=-1, name="Decay" },
    { cc_number=102, note=-1, name="Slide" },
    { cc_number=104, note=-1, name="Tuning" },
}

return {
    name = "Roland TB-03",
    cc   = cc,
}
