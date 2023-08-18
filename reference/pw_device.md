# Methods

## Subscribe params
Subscribe to parameter changes

Automatically emit param events for the given ids when
they are changed.

\param ids an array of param ids
\param n_ids the number of ids in \a ids

This requires X permissions on the device.

```c
static int device_marshal_subscribe_params(void *object, uint32_t *ids, uint32_t n_ids)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_proxy(proxy, PW_DEVICE_METHOD_SUBSCRIBE_PARAMS, NULL);

	spa_pod_builder_add_struct(b,
			SPA_POD_Array(sizeof(uint32_t), SPA_TYPE_Id, n_ids, ids));

	return pw_protocol_native_end_proxy(proxy, b);
}
```

```c
static int device_demarshal_subscribe_params(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct spa_pod_parser prs;
	uint32_t csize, ctype, n_ids;
	uint32_t *ids;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Array(&csize, &ctype, &n_ids, &ids)) < 0)
		return -EINVAL;

	if (ctype != SPA_TYPE_Id)
		return -EINVAL;

	return pw_resource_notify(resource, struct pw_device_methods, subscribe_params, 0,
			ids, n_ids);
}
```

## Enum params
Enumerate device parameters

Start enumeration of device parameters. For each param, a
param event will be emitted.

\param seq a sequence number to place in the reply
\param id the parameter id to enum or PW_ID_ANY for all
\param start the start index or 0 for the first param
\param num the maximum number of params to retrieve
\param filter a param filter or NULL

This requires X permissions on the device.

```c
static int device_marshal_enum_params(void *object, int seq,
		uint32_t id, uint32_t index, uint32_t num, const struct spa_pod *filter)
{
	struct pw_protocol_native_message *msg;
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_proxy(proxy, PW_DEVICE_METHOD_ENUM_PARAMS, &msg);

	spa_pod_builder_add_struct(b,
			SPA_POD_Int(SPA_RESULT_RETURN_ASYNC(msg->seq)),
			SPA_POD_Id(id),
			SPA_POD_Int(index),
			SPA_POD_Int(num),
			SPA_POD_Pod(filter));

	return pw_protocol_native_end_proxy(proxy, b);
}
```

```c
static int device_demarshal_enum_params(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct spa_pod_parser prs;
	uint32_t id, index, num;
	int seq;
	struct spa_pod *filter;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Int(&seq),
				SPA_POD_Id(&id),
				SPA_POD_Int(&index),
				SPA_POD_Int(&num),
				SPA_POD_Pod(&filter)) < 0)
		return -EINVAL;

	return pw_resource_notify(resource, struct pw_device_methods, enum_params, 0,
			seq, id, index, num, filter);
}
```

## Set param
Set a parameter on the device

\param id the parameter id to set
\param flags extra parameter flags
\param param the parameter to set

This requires W and X permissions on the device.

```c
static int device_marshal_set_param(void *object, uint32_t id, uint32_t flags,
		const struct spa_pod *param)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_proxy(proxy, PW_DEVICE_METHOD_SET_PARAM, NULL);

	spa_pod_builder_add_struct(b,
			SPA_POD_Id(id),
			SPA_POD_Int(flags),
			SPA_POD_Pod(param));
	return pw_protocol_native_end_proxy(proxy, b);
}
```

```c
static int device_demarshal_set_param(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct spa_pod_parser prs;
	uint32_t id, flags;
	struct spa_pod *param;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Id(&id),
				SPA_POD_Int(&flags),
				SPA_POD_Pod(&param)) < 0)
		return -EINVAL;

	return pw_resource_notify(resource, struct pw_device_methods, set_param, 0, id, flags, param);
}
```

# Events

## Info
Notify device info

\param info info about the device

```c
static void device_marshal_info(void *data, const struct pw_device_info *info)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;
	struct spa_pod_frame f;

	b = pw_protocol_native_begin_resource(resource, PW_DEVICE_EVENT_INFO, NULL);

	spa_pod_builder_push_struct(b, &f);
	spa_pod_builder_add(b,
			    SPA_POD_Int(info->id),
			    SPA_POD_Long(info->change_mask),
			    NULL);
	push_dict(b, info->change_mask & PW_DEVICE_CHANGE_MASK_PROPS ? info->props : NULL);
	push_params(b, info->n_params, info->params);
	spa_pod_builder_pop(b, &f);

	pw_protocol_native_end_resource(resource, b);
}
```

```c
static int device_demarshal_info(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_pod_parser prs;
	struct spa_pod_frame f[2];
	struct spa_dict props = SPA_DICT_INIT(NULL, 0);
	struct pw_device_info info = { .props = &props };

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_push_struct(&prs, &f[0]) < 0 ||
	    spa_pod_parser_get(&prs,
			SPA_POD_Int(&info.id),
			SPA_POD_Long(&info.change_mask), NULL) < 0)
		return -EINVAL;

	parse_dict_struct(&prs, &f[1], &props);
	parse_params_struct(&prs, &f[1], info.params, info.n_params);

	return pw_proxy_notify(proxy, struct pw_device_events, info, 0, &info);
}
```

## Param
Notify a device param

Event emitted as a result of the enum_params method.

\param seq the sequence number of the request
\param id the param id
\param index the param index
\param next the param index of the next param
\param param the parameter

```c
static void device_marshal_param(void *data, int seq, uint32_t id, uint32_t index, uint32_t next,
		const struct spa_pod *param)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_resource(resource, PW_DEVICE_EVENT_PARAM, NULL);

	spa_pod_builder_add_struct(b,
			SPA_POD_Int(seq),
			SPA_POD_Id(id),
			SPA_POD_Int(index),
			SPA_POD_Int(next),
			SPA_POD_Pod(param));

	pw_protocol_native_end_resource(resource, b);
}
```

```c
static int device_demarshal_param(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_pod_parser prs;
	uint32_t id, index, next;
	int seq;
	struct spa_pod *param;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Int(&seq),
				SPA_POD_Id(&id),
				SPA_POD_Int(&index),
				SPA_POD_Int(&next),
				SPA_POD_Pod(&param)) < 0)
		return -EINVAL;

	return pw_proxy_notify(proxy, struct pw_device_events, param, 0,
			seq, id, index, next, param);
}
```

# Other
```c
static const struct pw_device_methods pw_protocol_native_device_method_marshal = {
	PW_VERSION_DEVICE_METHODS,
	.add_listener = &device_method_marshal_add_listener,
	.subscribe_params = &device_marshal_subscribe_params,
	.enum_params = &device_marshal_enum_params,
	.set_param = &device_marshal_set_param,
};

static const struct pw_protocol_native_demarshal
pw_protocol_native_device_method_demarshal[PW_DEVICE_METHOD_NUM] = {
	[PW_DEVICE_METHOD_ADD_LISTENER] = { NULL, 0, },
	[PW_DEVICE_METHOD_SUBSCRIBE_PARAMS] = { &device_demarshal_subscribe_params, 0, },
	[PW_DEVICE_METHOD_ENUM_PARAMS] = { &device_demarshal_enum_params, 0, },
	[PW_DEVICE_METHOD_SET_PARAM] = { &device_demarshal_set_param, PW_PERM_W, },
};

static const struct pw_device_events pw_protocol_native_device_event_marshal = {
	PW_VERSION_DEVICE_EVENTS,
	.info = &device_marshal_info,
	.param = &device_marshal_param,
};

static const struct pw_protocol_native_demarshal
pw_protocol_native_device_event_demarshal[PW_DEVICE_EVENT_NUM] = {
	[PW_DEVICE_EVENT_INFO] = { &device_demarshal_info, 0, },
	[PW_DEVICE_EVENT_PARAM] = { &device_demarshal_param, 0, }
};

static const struct pw_protocol_marshal pw_protocol_native_device_marshal = {
	PW_TYPE_INTERFACE_Device,
	PW_VERSION_DEVICE,
	0,
	PW_DEVICE_METHOD_NUM,
	PW_DEVICE_EVENT_NUM,
	.client_marshal = &pw_protocol_native_device_method_marshal,
	.server_demarshal = pw_protocol_native_device_method_demarshal,
	.server_marshal = &pw_protocol_native_device_event_marshal,
	.client_demarshal = pw_protocol_native_device_event_demarshal,
};
```

