# Methods
- PW_MODULE_METHOD_ADD_LISTENER 0
- PW_MODULE_METHOD_NUM 1

# Events
- [PW_MODULE_EVENT_INFO 0](#info)
- PW_MODULE_EVENT_NUM 1

## Info
Notify module info

\param info info about the module

```c
static void module_marshal_info(void *data, const struct pw_module_info *info)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;
	struct spa_pod_frame f;

	b = pw_protocol_native_begin_resource(resource, PW_MODULE_EVENT_INFO, NULL);

	spa_pod_builder_push_struct(b, &f);
	spa_pod_builder_add(b,
			    SPA_POD_Int(info->id),
			    SPA_POD_String(info->name),
			    SPA_POD_String(info->filename),
			    SPA_POD_String(info->args),
			    SPA_POD_Long(info->change_mask),
			    NULL);
	push_dict(b, info->change_mask & PW_MODULE_CHANGE_MASK_PROPS ? info->props : NULL);
	spa_pod_builder_pop(b, &f);

	pw_protocol_native_end_resource(resource, b);
}
```

```c
static int module_demarshal_info(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_pod_parser prs;
	struct spa_pod_frame f[2];
	struct spa_dict props = SPA_DICT_INIT(NULL, 0);
	struct pw_module_info info = { .props = &props };

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_push_struct(&prs, &f[0]) < 0 ||
	    spa_pod_parser_get(&prs,
			SPA_POD_Int(&info.id),
			SPA_POD_String(&info.name),
			SPA_POD_String(&info.filename),
			SPA_POD_String(&info.args),
			SPA_POD_Long(&info.change_mask), NULL) < 0)
		return -EINVAL;

	parse_dict_struct(&prs, &f[1], &props);

	return pw_proxy_notify(proxy, struct pw_module_events, info, 0, &info);
}
```

