#include <ctype.h>
#include <getopt.h>
#include <locale.h>
#include <stdint.h>
#include <stdio.h>

#include "spa/debug/types.h"
#include "spa/utils/type.h"
#include <spa/control/control.h>
#include <spa/param/audio/format-utils.h>
#include <spa/param/props.h>
#include <spa/utils/defs.h>
#include <spa/utils/result.h>
#include <spa/utils/type-info.h>

#include <pipewire/filter.h>
#include <pipewire/pipewire.h>

// Function to check if a string starts with a given prefix
static bool starts_with(const char *str, const char *prefix) {
  while (*prefix) {
    if (*prefix != *str) {
      return false;
    }
    prefix++;
    str++;
  }
  return true;
}

// Function to strip the prefix
static const char *strip_prefix(const char *str, const char *prefix) {
  if (starts_with(str, prefix)) {
    size_t prefix_len = strlen(prefix);
    return str + prefix_len;
  }

  return NULL;
}

static char *camel_to_snake(const char *camel_in) {
  char *camel = malloc(strlen(camel_in));

  // TODO: Does not belong here
  {
    int j = 0;
    for (int i = 0; camel_in[i] != '\0'; i++) {
      if (camel_in[i] == ':') {
        camel[j++] = '_';
      } else {
        camel[j++] = camel_in[i];
      }
    }
    camel[j] = '\0';
  }

  char *snake = malloc(strlen(camel) * 2);

  int j = 0;
  for (int i = 0; camel[i] != '\0'; i++) {
    if (isupper(camel[i]) && (i == 0 || !isupper(camel[i - 1]))) {
      if (i != 0 && camel[i - 1] != '_') {
        snake[j++] = '_';
      }
      snake[j++] = tolower(camel[i]);
    } else {
      snake[j++] = camel[i];
    }
  }
  snake[j] = '\0';

  free(camel);
  return snake;
}

static const char *getter_name(const char *obj, const char *field) {
  return strip_prefix(strip_prefix(field, obj), ":");
}

static void print_get_fn_signature(const struct spa_type_info *info, const char *name,
                            const char *ret) {
  if (strcmp(name, "type") == 0) {
    name = "ty";
  }

  char *snake = camel_to_snake(name);

  printf("    /// %s\n", info->name);
  printf("    fn %s(&self) -> Option<%s> ", snake, ret);

  free(snake);
}

static void print_get_fn_body(uint32_t id, const char *as) {
  printf("{\n");
  printf("        self.get(%d)?.%s().ok()\n", id, as);
  printf("    }\n");
}

static void print_get_fn(const char *parent_full_name,
                  const struct spa_type_info *info, const char *ret,
                  const char *as) {
  const char *name = getter_name(parent_full_name, info->name);

  print_get_fn_signature(info, name, ret);
  print_get_fn_body(info->type, as);
}

static void info_to_rs_getter(const char *parent_full_name,
                       const struct spa_type_info *info) {
  const char *name = spa_debug_type_short_name(info->name);
  const struct spa_type_info *parent_info;

  if (name[0] == '\0')
    return;

  name = getter_name(parent_full_name, info->name);

  if (info->parent == SPA_TYPE_None) {
    return;
  }

  switch (info->parent) {
  case SPA_TYPE_Id: {
    print_get_fn(parent_full_name, info, "u32", "as_id");
    break;
  }
  case SPA_TYPE_Int: {
    print_get_fn(parent_full_name, info, "i32", "as_i32");
    break;
  }
  case SPA_TYPE_Long: {
    print_get_fn(parent_full_name, info, "i64", "as_i64");
    break;
  }
  case SPA_TYPE_Fd: {
    print_get_fn(parent_full_name, info, "i64", "as_fd");
    break;
  }
  case SPA_TYPE_Float: {
    print_get_fn(parent_full_name, info, "f32", "as_f32");
    break;
  }
  case SPA_TYPE_Double: {
    print_get_fn(parent_full_name, info, "f64", "as_f64");
    break;
  }
  case SPA_TYPE_Rectangle: {
    print_get_fn(parent_full_name, info, "SpaRectangle", "as_rectangle");
    break;
  }
  case SPA_TYPE_Fraction: {
    print_get_fn(parent_full_name, info, "SpaFraction", "as_fraction");
    break;
  }
  case SPA_TYPE_Bool: {
    print_get_fn(parent_full_name, info, "bool", "as_bool");
    break;
  }
  case SPA_TYPE_String: {
    print_get_fn(parent_full_name, info, "&BStr", "as_str");
    break;
  }
  case SPA_TYPE_Array: {
    parent_info = spa_debug_type_find(NULL, info->parent);
    printf("    /// TODO: returns: %s\n", parent_info->name);
    print_get_fn_signature(info, name, "OwnedPod");
    printf("{\n");
    printf("        Some(self.get(%d)?.to_owned())\n", info->type);
    printf("    }");
    break;
  }
  case SPA_TYPE_Pod: {
    print_get_fn_signature(info, name, "OwnedPod");
    printf("{\n");
    printf("        Some(self.get(%d)?.to_owned())\n", info->type);
    printf("    }");
    break;
  }
  case SPA_TYPE_Struct: {
    parent_info = spa_debug_type_find(NULL, info->parent);
    printf("    /// TODO: returns: %s\n", parent_info->name);

    print_get_fn_signature(info, name, "OwnedPod");
    printf("{\n");
    printf("        Some(self.get(%d)?.to_owned())\n", info->type);
    printf("    }");
    break;
  }
  default:
    parent_info = spa_debug_type_find(NULL, info->parent);

    printf("    /// TODO: returns: %s\n", parent_info->name);
    print_get_fn_signature(info, name, "OwnedPod");
    printf("{\n");
    printf("        Some(self.get(%d)?.to_owned())\n", info->type);
    printf("    }");
    break;
  }

  printf("\n\n");
}

static size_t type_info_count(size_t size) {
  return size / sizeof(struct spa_type_info);
}

struct type_deff {
  const char *name;
  const struct spa_type_info *info;
};

static void generate(struct type_deff r) {
  const char *name = spa_debug_type_short_name(r.name);
  const char *full_name = r.name;

  printf("/// %s\n", full_name);
  printf("struct %s;\n", name);
  printf("impl %s {\n", name);
  printf("    fn get(&self, id: u32) -> Option<PodDeserializer> { "
         "todo!(\"{id}\") }\n\n");
  while (r.info && r.info->name) {
    info_to_rs_getter(full_name, r.info);
    r.info++;
  }
  printf("}\n");
}

static const struct type_deff type_deffs[] = {
    {SPA_TYPE_INFO_PropInfo, spa_type_prop_info},
    {SPA_TYPE_INFO_Props, spa_type_props},
    {SPA_TYPE_INFO_Format, spa_type_format},
    {SPA_TYPE_INFO_PARAM_Buffers, spa_type_param_buffers},
    {SPA_TYPE_INFO_PARAM_Meta, spa_type_param_meta},
    {SPA_TYPE_INFO_PARAM_IO, spa_type_param_io},
    {SPA_TYPE_INFO_PARAM_Profile, spa_type_param_profile},
    {SPA_TYPE_INFO_PARAM_PortConfig, spa_type_param_port_config},
    {SPA_TYPE_INFO_PARAM_Route, spa_type_param_route},
    {SPA_TYPE_INFO_Profiler, spa_type_profiler},
    {SPA_TYPE_INFO_PARAM_Latency, spa_type_param_latency},
    {SPA_TYPE_INFO_PARAM_ProcessLatency, spa_type_param_process_latency},
    {SPA_TYPE_INFO_PARAM_Tag, spa_type_param_tag},
    {NULL, NULL},
};

void rust_out_run(void) {
  printf("use super::*;\n\n");

  const struct type_deff *deff = type_deffs;
  while (deff->info && deff->name) {
    generate(*deff);
    printf("\n");
    deff++;
  }
}
