/crates/fmod-studio-sys>     bindgen inc/fmod_studio.h -o generated_bindings1.rs --no-prepend-enum-name \
    --blocklist-file inc/fmod.h \
    --blocklist-file inc/fmod_codec.h \
    --blocklist-file inc/fmod_common.h \
    --blocklist-file inc/fmod_dsp.h \
    --blocklist-file inc/fmod_dsp_effects.h \
    --blocklist-file inc/fmod_errors.h \
    --blocklist-file inc/fmod_output.h \


/crates/fmod-studio-sys/inc> bindgen     fmod_studio.h -o generated_bindings2.rs --no-prepend-enum-name \
    --blocklist-file     fmod.h \
    --blocklist-file     fmod_codec.h \
    --blocklist-file     fmod_common.h \
    --blocklist-file     fmod_dsp.h \
    --blocklist-file     fmod_dsp_effects.h \
    --blocklist-file     fmod_errors.h \
    --blocklist-file     fmod_output.h \
