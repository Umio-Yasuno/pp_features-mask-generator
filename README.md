# pfg
pp_features mask generator for AMDGPU

 * https://github.com/torvalds/linux/blob/master/drivers/gpu/drm/amd/pm/amdgpu_pm.c#L897-L912

## Usage
### dump pp_features
```
cargo run -- /sys/class/drm/card0/device
```

### generate a mask with `DPM_GFX_GPO` enabled
```
cargo run -- /sys/class/drm/card0/device +DPM_GFX_GPO
```

### generate a mask with `DPM_GFX_GPO` disabled
```
cargo run -- /sys/class/drm/card0/device -DPM_GFX_GPO
```

### write a pp_features mask
```
# echo <mask> > /sys/class/drm/card0/device/pp_features
```
