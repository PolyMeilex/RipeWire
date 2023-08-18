# Methods

## Subscribe params
Subscribe to parameter changes

Automatically emit param events for the given ids when
they are changed.

\param ids an array of param ids
\param n_ids the number of ids in \a ids

This requires X permissions on the node.

```c
static int node_marshal_subscribe_params(void *object, uint32_t *ids, uint32_t n_ids)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_proxy(proxy, PW_NODE_METHOD_SUBSCRIBE_PARAMS, NULL);

	spa_pod_builder_add_struct(b,
			SPA_POD_Array(sizeof(uint32_t), SPA_TYPE_Id, n_ids, ids));

	return pw_protocol_native_end_proxy(proxy, b);
}
```

```c
static int node_demarshal_subscribe_params(void *object, const struct pw_protocol_native_message *msg)
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

	return pw_resource_notify(resource, struct pw_node_methods, subscribe_params, 0,
			ids, n_ids);
}
```

## Enum params
Enumerate node parameters

Start enumeration of node parameters. For each param, a
param event will be emitted.

\param seq a sequence number to place in the reply
\param id the parameter id to enum or PW_ID_ANY for all
\param start the start index or 0 for the first param
\param num the maximum number of params to retrieve
\param filter a param filter or NULL

This requires X permissions on the node.
```c
static int node_marshal_enum_params(void *object, int seq, uint32_t id,
		uint32_t index, uint32_t num, const struct spa_pod *filter)
{
	struct pw_protocol_native_message *msg;
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_proxy(proxy, PW_NODE_METHOD_ENUM_PARAMS, &msg);

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
static int node_demarshal_enum_params(void *object, const struct pw_protocol_native_message *msg)
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

	return pw_resource_notify(resource, struct pw_node_methods, enum_params, 0,
			seq, id, index, num, filter);
}
```

## Set param
Set a parameter on the node

\param id the parameter id to set
\param flags extra parameter flags
\param param the parameter to set

This requires X and W permissions on the node.
```c
static int node_marshal_set_param(void *object, uint32_t id, uint32_t flags,
		const struct spa_pod *param)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_proxy(proxy, PW_NODE_METHOD_SET_PARAM, NULL);

	spa_pod_builder_add_struct(b,
			SPA_POD_Id(id),
			SPA_POD_Int(flags),
			SPA_POD_Pod(param));
	return pw_protocol_native_end_proxy(proxy, b);
}
```

```c
static int node_demarshal_set_param(void *object, const struct pw_protocol_native_message *msg)
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

	return pw_resource_notify(resource, struct pw_node_methods, set_param, 0, id, flags, param);
}
```

## Send command
Send a command to the node

\param command the command to send

This requires X and W permissions on the node.
```c
static int node_marshal_send_command(void *object, const struct spa_command *command)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_proxy(proxy, PW_NODE_METHOD_SEND_COMMAND, NULL);
	spa_pod_builder_add_struct(b,
			SPA_POD_Pod(command));
	return pw_protocol_native_end_proxy(proxy, b);
}
```

```c
static int node_demarshal_send_command(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct spa_pod_parser prs;
	const struct spa_command *command;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Pod(&command)) < 0)
		return -EINVAL;

	return pw_resource_notify(resource, struct pw_node_methods, send_command, 0, command);
}
```

# Events

## Info
Notify node info

\param info info about the node
```c
static void node_marshal_info(void *data, const struct pw_node_info *info)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;
	struct spa_pod_frame f;

	b = pw_protocol_native_begin_resource(resource, PW_NODE_EVENT_INFO, NULL);

	spa_pod_builder_push_struct(b, &f);
	spa_pod_builder_add(b,
			    SPA_POD_Int(info->id),
			    SPA_POD_Int(info->max_input_ports),
			    SPA_POD_Int(info->max_output_ports),
			    SPA_POD_Long(info->change_mask),
			    SPA_POD_Int(info->n_input_ports),
			    SPA_POD_Int(info->n_output_ports),
			    SPA_POD_Id(info->state),
			    SPA_POD_String(info->error),
			    NULL);
	push_dict(b, info->change_mask & PW_NODE_CHANGE_MASK_PROPS ? info->props : NULL);
	push_params(b, info->n_params, info->params);
	spa_pod_builder_pop(b, &f);

	pw_protocol_native_end_resource(resource, b);
}
```

```c
static int node_demarshal_info(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_pod_parser prs;
	struct spa_pod_frame f[2];
	struct spa_dict props = SPA_DICT_INIT(NULL, 0);
	struct pw_node_info info = { .props = &props };

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_push_struct(&prs, &f[0]) < 0 ||
	    spa_pod_parser_get(&prs,
			SPA_POD_Int(&info.id),
			SPA_POD_Int(&info.max_input_ports),
			SPA_POD_Int(&info.max_output_ports),
			SPA_POD_Long(&info.change_mask),
			SPA_POD_Int(&info.n_input_ports),
			SPA_POD_Int(&info.n_output_ports),
			SPA_POD_Id(&info.state),
			SPA_POD_String(&info.error), NULL) < 0)
		return -EINVAL;

	parse_dict_struct(&prs, &f[1], &props);
	parse_params_struct(&prs, &f[1], info.params, info.n_params);

	return pw_proxy_notify(proxy, struct pw_node_events, info, 0, &info);
}
```

## Param
Notify a node param

Event emitted as a result of the enum_params method.

\param seq the sequence number of the request
\param id the param id
\param index the param index
\param next the param index of the next param
\param param the parameter
```c
static void node_marshal_param(void *data, int seq, uint32_t id,
		uint32_t index, uint32_t next, const struct spa_pod *param)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_resource(resource, PW_NODE_EVENT_PARAM, NULL);

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
static int node_demarshal_param(void *data, const struct pw_protocol_native_message *msg)
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

	return pw_proxy_notify(proxy, struct pw_node_events, param, 0,
			seq, id, index, next, param);
}
```

# Other
```c
static const struct pw_node_methods pw_protocol_native_node_method_marshal = {
	PW_VERSION_NODE_METHODS,
	.add_listener = &node_method_marshal_add_listener,
	.subscribe_params = &node_marshal_subscribe_params,
	.enum_params = &node_marshal_enum_params,
	.set_param = &node_marshal_set_param,
	.send_command = &node_marshal_send_command,
};

static const struct pw_protocol_native_demarshal
pw_protocol_native_node_method_demarshal[PW_NODE_METHOD_NUM] =
{
	[PW_NODE_METHOD_ADD_LISTENER] = { NULL, 0, },
	[PW_NODE_METHOD_SUBSCRIBE_PARAMS] = { &node_demarshal_subscribe_params, 0, },
	[PW_NODE_METHOD_ENUM_PARAMS] = { &node_demarshal_enum_params, 0, },
	[PW_NODE_METHOD_SET_PARAM] = { &node_demarshal_set_param, PW_PERM_W, },
	[PW_NODE_METHOD_SEND_COMMAND] = { &node_demarshal_send_command, PW_PERM_W, },
};

static const struct pw_node_events pw_protocol_native_node_event_marshal = {
	PW_VERSION_NODE_EVENTS,
	.info = &node_marshal_info,
	.param = &node_marshal_param,
};

static const struct pw_protocol_native_demarshal
pw_protocol_native_node_event_demarshal[PW_NODE_EVENT_NUM] = {
	[PW_NODE_EVENT_INFO] = { &node_demarshal_info, 0, },
	[PW_NODE_EVENT_PARAM] = { &node_demarshal_param, 0, }
};

static const struct pw_protocol_marshal pw_protocol_native_node_marshal = {
	PW_TYPE_INTERFACE_Node,
	PW_VERSION_NODE,
	0,
	PW_NODE_METHOD_NUM,
	PW_NODE_EVENT_NUM,
	.client_marshal = &pw_protocol_native_node_method_marshal,
	.server_demarshal = pw_protocol_native_node_method_demarshal,
	.server_marshal = &pw_protocol_native_node_event_marshal,
	.client_demarshal = pw_protocol_native_node_event_demarshal,
};
```
