#include <spa/utils/type-info.h>

struct type_deff {
  const char *name;
  const struct spa_type_info *info;
};

static int ident_level = 0;
static void push_ident(void) { ident_level += 1; }
static void pop_ident(void) { ident_level -= 1; }
static void print_ident(void) {
  for (int i = 0; i < ident_level; i++) {
    printf("  ");
  }
}

static void print_obj_key(const char *key) { printf("\"%s\": ", key); }

static void generate_info(const struct spa_type_info *info);

static void generate_property(const struct spa_type_info *info) {
  printf("{ ");

  print_obj_key("type");
  printf("%d, ", info->type);

  print_obj_key("parent");
  printf("%d, ", info->parent);

  print_obj_key("name");
  printf("\"%s\"", info->name);

  if (info->values) {
    printf(", ");
    printf("\n");

    push_ident();
    {
      print_ident();
      print_obj_key("values");
      generate_info(info->values);
      printf("\n");
    }
    pop_ident();

    print_ident();
  } else {
    printf(" ");
  }

  printf("}");
}

static void generate_info(const struct spa_type_info *info) {
  printf("[\n");

  while (true) {
    push_ident();
    print_ident();
    generate_property(info);
    pop_ident();

    info++;

    if (info && info->name) {
      printf(",\n");
    } else {
      break;
    }
  }

  printf("\n");
  print_ident();
  printf("]");
}

static void generate(struct type_deff r) {
  printf("{\n");

  print_ident();
  print_obj_key("name");
  printf("\"%s\", \n", r.name);

  print_ident();
  print_obj_key("properties");
  generate_info(r.info);
  printf("\n");

  printf("}");
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

void json_out_run(void) {
  const struct type_deff *deff = type_deffs;
  printf("[\n");
  while (true) {
    push_ident();
    generate(*deff);
    pop_ident();
    deff++;

    if (deff->info && deff->name) {
      printf(",\n");
    } else {
      printf("\n");
      break;
    }
  }
  printf("]\n");
}
