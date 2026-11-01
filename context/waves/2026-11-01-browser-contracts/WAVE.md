# Browser contract workbench

Status: consumer verification complete locally; hosted release qualification pending.
VTRACE stages 5–6 apply; downstream stage 7 remains deferred.

The approved descriptor and collection interfaces already passed stages 0–4.
This wave exposes those existing APIs in a standalone browser consumer. It does
not change core models, derive behavior, registry evidence, or runtime authority.

Users supply descriptor/collection JSON, inspect native validation and approved
profile compatibility, then download generated artifacts and exact input.
Collection inventory and the existing review adapter are included. Unknown JSON
fields must be disclosed rather than silently presented as validated metadata.

Execution: [pulse 01](pulses/pulse-01.md). Broader adoption validation remains
deferred; browser parity is verification of this consumer only.
