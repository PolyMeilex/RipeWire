project('c_obj_gen', 'c',
  version : '0.1',
  default_options : ['warning_level=3']
)

pipewire_dep = dependency('libpipewire-0.3')

subdir('src')

exe = executable('c_obj_gen', 
  sources,
  install : true,
  dependencies: [pipewire_dep],
)

test('basic', exe)
