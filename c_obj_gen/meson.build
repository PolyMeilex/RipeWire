project('c_obj_gen', 'c',
  version : '0.1',
  default_options : ['warning_level=3']
)

spa_lib_dir = '/home/poly/dev/pipewire/_install/lib64/spa-0.2'
spa_header_dir = '/home/poly/dev/pipewire/_install/include/spa-0.2'

pipewire_lib_dir = '/home/poly/dev/pipewire/_install/lib64/pipewire-0.3'
pipewire_header_dir = '/home/poly/dev/pipewire/_install/include/pipewire-0.3'

pipewire_dep = declare_dependency(
    link_args : ['-L' + pipewire_lib_dir, '-lpipewire-0.3', '-L' + spa_lib_dir, '-lpipewire-0.3'],
    include_directories : include_directories(spa_header_dir, pipewire_header_dir))

# pipewire_dep = dependency('libpipewire-0.3')

subdir('src')

exe = executable('c_obj_gen', 
  sources,
  install : true,
  dependencies: [pipewire_dep],
)

test('basic', exe)
