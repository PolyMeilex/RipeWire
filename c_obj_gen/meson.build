project('c_obj_gen', 'c',
  version : '0.1',
  default_options : ['warning_level=3']
)

pipewire_dep = dependency('libpipewire-0.3')

exe = executable('c_obj_gen', 
  'c_obj_gen.c',
  install : true,
  dependencies: [pipewire_dep],
)

test('basic', exe)
