# Methods

## Hello
Start a conversation with the server. This will send
the core info and will destroy all resources for the client
(except the core and client resource).

```c
static int core_method_marshal_hello(void *object, uint32_t version)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_proxy(proxy, PW_CORE_METHOD_HELLO, NULL);

	spa_pod_builder_add_struct(b,
			SPA_POD_Int(version));

	return pw_protocol_native_end_proxy(proxy, b);
}
```
```c
static int core_method_demarshal_hello(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct spa_pod_parser prs;
	uint32_t version;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Int(&version)) < 0)
		return -EINVAL;

	return pw_resource_notify(resource, struct pw_core_methods, hello, 0, version);
}
```

## Sync
Do server roundtrip

Ask the server to emit the 'done' event with \a seq.

Since methods are handled in-order and events are delivered
in-order, this can be used as a barrier to ensure all previous
methods and the resulting events have been handled.

seq - the seq number passed to the done event

```c
static int core_method_marshal_sync(void *object, uint32_t id, int seq)
{
	struct pw_protocol_native_message *msg;
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_proxy(proxy, PW_CORE_METHOD_SYNC, &msg);

	spa_pod_builder_add_struct(b,
			SPA_POD_Int(id),
			SPA_POD_Int(SPA_RESULT_RETURN_ASYNC(msg->seq)));

	return pw_protocol_native_end_proxy(proxy, b);
}
```
```c
static int core_method_demarshal_sync(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct spa_pod_parser prs;
	uint32_t id, seq;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Int(&id),
				SPA_POD_Int(&seq)) < 0)
		return -EINVAL;

	return pw_resource_notify(resource, struct pw_core_methods, sync, 0, id, seq);
}
```

## Pong
Reply to a server ping event.

Reply to the server ping event with the same seq.

seq - the seq number received in the ping event

```c
static int core_method_marshal_pong(void *object, uint32_t id, int seq)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_proxy(proxy, PW_CORE_METHOD_PONG, NULL);

	spa_pod_builder_add_struct(b,
			SPA_POD_Int(id),
			SPA_POD_Int(seq));

	return pw_protocol_native_end_proxy(proxy, b);
}
```
```c
static int core_method_demarshal_pong(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct spa_pod_parser prs;
	uint32_t id, seq;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Int(&id),
				SPA_POD_Int(&seq)) < 0)
		return -EINVAL;

	return pw_resource_notify(resource, struct pw_core_methods, pong, 0, id, seq);
}
```

## Error
Fatal error event

The error method is sent out when a fatal (non-recoverable)
error has occurred. The id argument is the proxy object where
the error occurred, most often in response to an event on that
object. The message is a brief description of the error,
for (debugging) convenience.

This method is usually also emitted on the resource object with
id.

id - object where the error occurred
res - error code
message - error description

```c
static int core_method_marshal_error(void *object, uint32_t id, int seq, int res, const char *error)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_proxy(proxy, PW_CORE_METHOD_ERROR, NULL);

	spa_pod_builder_add_struct(b,
			       SPA_POD_Int(id),
			       SPA_POD_Int(seq),
			       SPA_POD_Int(res),
			       SPA_POD_String(error));

	return pw_protocol_native_end_proxy(proxy, b);
}
```
```c
static int core_event_demarshal_error(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_pod_parser prs;
	uint32_t id, res;
	int seq;
	const char *error;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
			SPA_POD_Int(&id),
			SPA_POD_Int(&seq),
			SPA_POD_Int(&res),
			SPA_POD_String(&error)) < 0)
		return -EINVAL;

	return pw_proxy_notify(proxy, struct pw_core_events, error, 0, id, seq, res, error);
}
```

## Get registry
Get the registry object

Create a registry object that allows the client to list and bind
the global objects available from the PipeWire server

version - the client version
user_data_size - extra size

```c
static struct pw_registry * core_method_marshal_get_registry(void *object,
		uint32_t version, size_t user_data_size)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;
	struct pw_proxy *res;
	uint32_t new_id;

	res = pw_proxy_new(object, PW_TYPE_INTERFACE_Registry, version, user_data_size);
	if (res == NULL)
		return NULL;

	new_id = pw_proxy_get_id(res);

	b = pw_protocol_native_begin_proxy(proxy, PW_CORE_METHOD_GET_REGISTRY, NULL);

	spa_pod_builder_add_struct(b,
		       SPA_POD_Int(version),
		       SPA_POD_Int(new_id));

	pw_protocol_native_end_proxy(proxy, b);

	return (struct pw_registry *) res;
}
```
```c
static int core_method_demarshal_get_registry(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct spa_pod_parser prs;
	int32_t version, new_id;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Int(&version),
				SPA_POD_Int(&new_id)) < 0)
		return -EINVAL;

	return pw_resource_notify(resource, struct pw_core_methods, get_registry, 0, version, new_id);
}
```

## Create object
Create a new object on the PipeWire server from a factory.

factory_name - the factory name to use
obj_type - the interface to bind to
version - the version of the interface
properties - extra properties

```c
static void *
core_method_marshal_create_object(void *object,
			   const char *factory_name,
			   const char *type, uint32_t version,
			   const struct spa_dict *props, size_t user_data_size)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;
	struct spa_pod_frame f;
	struct pw_proxy *res;
	uint32_t new_id;

	res = pw_proxy_new(object, type, version, user_data_size);
	if (res == NULL)
		return NULL;

	new_id = pw_proxy_get_id(res);

	b = pw_protocol_native_begin_proxy(proxy, PW_CORE_METHOD_CREATE_OBJECT, NULL);

	spa_pod_builder_push_struct(b, &f);
	spa_pod_builder_add(b,
			SPA_POD_String(factory_name),
			SPA_POD_String(type),
			SPA_POD_Int(version),
			NULL);
	push_dict(b, props);
	spa_pod_builder_int(b, new_id);
	spa_pod_builder_pop(b, &f);

	pw_protocol_native_end_proxy(proxy, b);

	return (void *)res;
}
```
```c
static int core_method_demarshal_create_object(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct spa_pod_parser prs;
	struct spa_pod_frame f[2];
	uint32_t version, new_id;
	const char *factory_name, *type;
	struct spa_dict props = SPA_DICT_INIT(NULL, 0);

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_push_struct(&prs, &f[0]) < 0 ||
	    spa_pod_parser_get(&prs,
			SPA_POD_String(&factory_name),
			SPA_POD_String(&type),
			SPA_POD_Int(&version),
			NULL) < 0)
		return -EINVAL;

	parse_dict_struct(&prs, &f[1], &props);

	if (spa_pod_parser_get(&prs,
			SPA_POD_Int(&new_id), NULL) < 0)
		return -EINVAL;

	return pw_resource_notify(resource, struct pw_core_methods, create_object, 0, factory_name,
								      type, version,
								      &props, new_id);
}
```

## Destroy
Destroy an resource

Destroy the server resource

id - id of object to destroy

```c
static int
core_method_marshal_destroy(void *object, void *p)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;
	uint32_t id = pw_proxy_get_id(p);

	b = pw_protocol_native_begin_proxy(proxy, PW_CORE_METHOD_DESTROY, NULL);

	spa_pod_builder_add_struct(b,
			SPA_POD_Int(id));

	return pw_protocol_native_end_proxy(proxy, b);
}
```
```c
static int core_method_demarshal_destroy(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct pw_impl_client *client = pw_resource_get_client(resource);
	struct pw_resource *r;
	struct spa_pod_parser prs;
	uint32_t id;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
			SPA_POD_Int(&id)) < 0)
		return -EINVAL;

	pw_log_debug("client %p: destroy resource %u", client, id);

	if ((r = pw_impl_client_find_resource(client, id)) == NULL)
		goto no_resource;

	return pw_resource_notify(resource, struct pw_core_methods, destroy, 0, r);

      no_resource:
	pw_log_debug("client %p: unknown resource %u op:%u", client, id, msg->opcode);
	pw_resource_errorf(resource, -ENOENT, "unknown resource %d op:%u", id, msg->opcode);
	return 0;
}
```

# Events

## Info
This event is emitted when first bound to the core or when the
hello method is called.

```c
static void core_event_marshal_info(void *data, const struct pw_core_info *info)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;
	struct spa_pod_frame f;

	b = pw_protocol_native_begin_resource(resource, PW_CORE_EVENT_INFO, NULL);

	spa_pod_builder_push_struct(b, &f);
	spa_pod_builder_add(b,
			    SPA_POD_Int(info->id),
			    SPA_POD_Int(info->cookie),
			    SPA_POD_String(info->user_name),
			    SPA_POD_String(info->host_name),
			    SPA_POD_String(info->version),
			    SPA_POD_String(info->name),
			    SPA_POD_Long(info->change_mask),
			    NULL);
	push_dict(b, info->change_mask & PW_CORE_CHANGE_MASK_PROPS ? info->props : NULL);
	spa_pod_builder_pop(b, &f);

	pw_protocol_native_end_resource(resource, b);
}
```
```c
static int core_event_demarshal_info(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_dict props = SPA_DICT_INIT(NULL, 0);
	struct pw_core_info info = { .props = &props };
	struct spa_pod_frame f[2];
	struct spa_pod_parser prs;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_push_struct(&prs, &f[0]) < 0)
		return -EINVAL;
	if (spa_pod_parser_get(&prs,
			 SPA_POD_Int(&info.id),
			 SPA_POD_Int(&info.cookie),
			 SPA_POD_String(&info.user_name),
			 SPA_POD_String(&info.host_name),
			 SPA_POD_String(&info.version),
			 SPA_POD_String(&info.name),
			 SPA_POD_Long(&info.change_mask), NULL) < 0)
		return -EINVAL;


	parse_dict_struct(&prs, &f[1], &props);

	return pw_proxy_notify(proxy, struct pw_core_events, info, 0, &info);
}
```

## Done

```c
static void core_event_marshal_done(void *data, uint32_t id, int seq)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_resource(resource, PW_CORE_EVENT_DONE, NULL);

	spa_pod_builder_add_struct(b,
			SPA_POD_Int(id),
			SPA_POD_Int(seq));

	pw_protocol_native_end_resource(resource, b);
}
```
```c
static int core_event_demarshal_done(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_pod_parser prs;
	uint32_t id, seq;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Int(&id),
				SPA_POD_Int(&seq)) < 0)
		return -EINVAL;

	if (id == SPA_ID_INVALID)
		return 0;

	return pw_proxy_notify(proxy, struct pw_core_events, done, 0, id, seq);
}

```

## Ping

```c
static void core_event_marshal_ping(void *data, uint32_t id, int seq)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;
	struct pw_protocol_native_message *msg;

	b = pw_protocol_native_begin_resource(resource, PW_CORE_EVENT_PING, &msg);

	spa_pod_builder_add_struct(b,
			SPA_POD_Int(id),
			SPA_POD_Int(SPA_RESULT_RETURN_ASYNC(msg->seq)));

	pw_protocol_native_end_resource(resource, b);
}
```
```c
static int core_event_demarshal_ping(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_pod_parser prs;
	uint32_t id, seq;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Int(&id),
				SPA_POD_Int(&seq)) < 0)
		return -EINVAL;

	return pw_proxy_notify(proxy, struct pw_core_events, ping, 0, id, seq);
}
```

## Remove id

```c
static void core_event_marshal_remove_id(void *data, uint32_t id)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_resource(resource, PW_CORE_EVENT_REMOVE_ID, NULL);

	spa_pod_builder_add_struct(b,
			SPA_POD_Int(id));

	pw_protocol_native_end_resource(resource, b);
}
```
```c
static int core_event_demarshal_remove_id(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_pod_parser prs;
	uint32_t id;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs, SPA_POD_Int(&id)) < 0)
		return -EINVAL;

	return pw_proxy_notify(proxy, struct pw_core_events, remove_id, 0, id);
}
```

## Bound id

```c
static void core_event_marshal_bound_id(void *data, uint32_t id, uint32_t global_id)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_resource(resource, PW_CORE_EVENT_BOUND_ID, NULL);

	spa_pod_builder_add_struct(b,
			SPA_POD_Int(id),
			SPA_POD_Int(global_id));

	pw_protocol_native_end_resource(resource, b);
}
```
```c
static int core_event_demarshal_bound_id(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_pod_parser prs;
	uint32_t id, global_id;
	struct spa_dict props = SPA_DICT_INIT(NULL, 0);

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Int(&id),
				SPA_POD_Int(&global_id)) < 0)
		return -EINVAL;

	/* old client / old/new server -> bound_id
	 * new client / old server     -> bound_id + bound_props (in case it's using bound_props only) */
	pw_proxy_notify(proxy, struct pw_core_events, bound_id, 0, id, global_id);
	return pw_proxy_notify(proxy, struct pw_core_events, bound_props, 1, id, global_id, &props);
}
```

## Bound props

```c
static void core_event_marshal_bound_props(void *data, uint32_t id, uint32_t global_id, const struct spa_dict *props)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;
	struct spa_pod_frame f;

	b = pw_protocol_native_begin_resource(resource, PW_CORE_EVENT_BOUND_PROPS, NULL);

	spa_pod_builder_push_struct(b, &f);
	spa_pod_builder_add(b,
			SPA_POD_Int(id),
			SPA_POD_Int(global_id),
			NULL);
	push_dict(b, props);
	spa_pod_builder_pop(b, &f);

	pw_protocol_native_end_resource(resource, b);
}
```
```c
static int core_event_demarshal_bound_props(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_pod_parser prs;
	uint32_t id, global_id;
	struct spa_pod_frame f[2];
	struct spa_dict props = SPA_DICT_INIT(NULL, 0);

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_push_struct(&prs, &f[0]) < 0)
		return -EINVAL;
	if (spa_pod_parser_get(&prs,
				SPA_POD_Int(&id),
				SPA_POD_Int(&global_id), NULL) < 0)
		return -EINVAL;

	parse_dict_struct(&prs, &f[1], &props);

	/* new client / new server -> bound_props + bound_id (in case it's not using bound_props yet) */
	pw_proxy_notify(proxy, struct pw_core_events, bound_id, 0, id, global_id);
	return pw_proxy_notify(proxy, struct pw_core_events, bound_props, 1, id, global_id, &props);
}
```

## Add mem

```c
static void core_event_marshal_add_mem(void *data, uint32_t id, uint32_t type, int fd, uint32_t flags)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_resource(resource, PW_CORE_EVENT_ADD_MEM, NULL);

	spa_pod_builder_add_struct(b,
			SPA_POD_Int(id),
			SPA_POD_Id(type),
			SPA_POD_Fd(pw_protocol_native_add_resource_fd(resource, fd)),
			SPA_POD_Int(flags));

	pw_protocol_native_end_resource(resource, b);
}
```
```c
static int core_event_demarshal_add_mem(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_pod_parser prs;
	uint32_t id, type, flags;
	int64_t idx;
	int fd;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Int(&id),
				SPA_POD_Id(&type),
				SPA_POD_Fd(&idx),
				SPA_POD_Int(&flags)) < 0)
		return -EINVAL;

	fd = pw_protocol_native_get_proxy_fd(proxy, idx);

	return pw_proxy_notify(proxy, struct pw_core_events, add_mem, 0, id, type, fd, flags);
}
```

## Remove mem

```c
static void core_event_marshal_remove_mem(void *data, uint32_t id)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_resource(resource, PW_CORE_EVENT_REMOVE_MEM, NULL);

	spa_pod_builder_add_struct(b,
			SPA_POD_Int(id));

	pw_protocol_native_end_resource(resource, b);
}
```
```c
static int core_event_demarshal_remove_mem(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_pod_parser prs;
	uint32_t id;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Int(&id)) < 0)
		return -EINVAL;

	return pw_proxy_notify(proxy, struct pw_core_events, remove_mem, 0, id);
}
```

## Error

```c
static void core_event_marshal_error(void *data, uint32_t id, int seq, int res, const char *error)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_resource(resource, PW_CORE_EVENT_ERROR, NULL);

	spa_pod_builder_add_struct(b,
			       SPA_POD_Int(id),
			       SPA_POD_Int(seq),
			       SPA_POD_Int(res),
			       SPA_POD_String(error));

	pw_protocol_native_end_resource(resource, b);
}
```
```c
static int core_method_demarshal_error(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct spa_pod_parser prs;
	uint32_t id, res;
	int seq;
	const char *error;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
			SPA_POD_Int(&id),
			SPA_POD_Int(&seq),
			SPA_POD_Int(&res),
			SPA_POD_String(&error)) < 0)
		return -EINVAL;

	return pw_resource_notify(resource, struct pw_core_methods, error, 0, id, seq, res, error);
}
```

# Dictionary stuff
```c
static inline void push_item(struct spa_pod_builder *b, const struct spa_dict_item *item)
{
	const char *str;
	spa_pod_builder_string(b, item->key);
	str = item->value;
	if (spa_strstartswith(str, "pointer:"))
		str = "";
	spa_pod_builder_string(b, str);
}

static void push_dict(struct spa_pod_builder *b, const struct spa_dict *dict)
{
	uint32_t i, n_items;
	struct spa_pod_frame f;

	n_items = dict ? dict->n_items : 0;

	spa_pod_builder_push_struct(b, &f);
	spa_pod_builder_int(b, n_items);
	for (i = 0; i < n_items; i++)
		push_item(b, &dict->items[i]);
	spa_pod_builder_pop(b, &f);
}

static inline int parse_item(struct spa_pod_parser *prs, struct spa_dict_item *item)
{
	int res;
	if ((res = spa_pod_parser_get(prs,
		       SPA_POD_String(&item->key),
		       SPA_POD_String(&item->value),
		       NULL)) < 0)
		return res;
	if (spa_strstartswith(item->value, "pointer:"))
		item->value = "";
	return 0;
}

#define parse_dict(prs,d)									\
do {												\
	if (spa_pod_parser_get(prs,								\
			 SPA_POD_Int(&(d)->n_items), NULL) < 0)					\
		return -EINVAL;									\
	(d)->items = NULL;									\
	if ((d)->n_items > 0) {									\
		uint32_t i;									\
		if ((d)->n_items > MAX_DICT)							\
			return -ENOSPC;								\
		(d)->items = alloca((d)->n_items * sizeof(struct spa_dict_item));		\
		for (i = 0; i < (d)->n_items; i++) {						\
			if (parse_item(prs, (struct spa_dict_item *) &(d)->items[i]) < 0)	\
				return -EINVAL;							\
		}										\
	}											\
} while(0)

#define parse_dict_struct(prs,f,dict)						\
do {										\
	if (spa_pod_parser_push_struct(prs, f) < 0)				\
		return -EINVAL;							\
	parse_dict(prs, dict);							\
	spa_pod_parser_pop(prs, f);						\
} while(0)

static void push_params(struct spa_pod_builder *b, uint32_t n_params,
		const struct spa_param_info *params)
{
	uint32_t i;
	struct spa_pod_frame f;

	spa_pod_builder_push_struct(b, &f);
	spa_pod_builder_int(b, n_params);
	for (i = 0; i < n_params; i++) {
		spa_pod_builder_id(b, params[i].id);
		spa_pod_builder_int(b, params[i].flags);
	}
	spa_pod_builder_pop(b, &f);
}


#define parse_params_struct(prs,f,params,n_params)					\
do {											\
	if (spa_pod_parser_push_struct(prs, f) < 0 ||					\
	    spa_pod_parser_get(prs,							\
			       SPA_POD_Int(&(n_params)), NULL) < 0)			\
		return -EINVAL;								\
	(params) = NULL;									\
	if ((n_params) > 0) {								\
		uint32_t i;								\
		if ((n_params) > MAX_PARAM_INFO)						\
			return -ENOSPC;							\
		(params) = alloca((n_params) * sizeof(struct spa_param_info));		\
		for (i = 0; i < (n_params); i++) {					\
			if (spa_pod_parser_get(prs,					\
				       SPA_POD_Id(&(params)[i].id),			\
				       SPA_POD_Int(&(params)[i].flags), NULL) < 0)	\
				return -EINVAL;						\
		}									\
	}										\
	spa_pod_parser_pop(prs, f);							\
} while(0)


#define parse_permissions_struct(prs,f,n_permissions,permissions)				\
do {												\
	if (spa_pod_parser_push_struct(prs, f) < 0 ||						\
	    spa_pod_parser_get(prs,								\
		    SPA_POD_Int(&(n_permissions)), NULL) < 0)					\
		return -EINVAL;									\
	(permissions) = NULL;									\
	if ((n_permissions) > 0) {								\
		uint32_t i;									\
		if ((n_permissions) > MAX_PERMISSIONS)						\
			return -ENOSPC;								\
		(permissions) = alloca((n_permissions) * sizeof(struct pw_permission));		\
		for (i = 0; i < (n_permissions); i++) {						\
			if (spa_pod_parser_get(prs,						\
					SPA_POD_Int(&(permissions)[i].id),			\
					SPA_POD_Int(&(permissions)[i].permissions), NULL) < 0)	\
				return -EINVAL;							\
		}										\
	}											\
	spa_pod_parser_pop(prs, f);								\
} while(0)
```

# Other

```c
static const struct pw_core_methods pw_protocol_native_core_method_marshal = {
	PW_VERSION_CORE_METHODS,
	.add_listener = &core_method_marshal_add_listener,
	.hello = &core_method_marshal_hello,
	.sync = &core_method_marshal_sync,
	.pong = &core_method_marshal_pong,
	.error = &core_method_marshal_error,
	.get_registry = &core_method_marshal_get_registry,
	.create_object = &core_method_marshal_create_object,
	.destroy = &core_method_marshal_destroy,
};

static const struct pw_protocol_native_demarshal pw_protocol_native_core_method_demarshal[PW_CORE_METHOD_NUM] = {
	[PW_CORE_METHOD_ADD_LISTENER] = { NULL, 0, },
	[PW_CORE_METHOD_HELLO] = { &core_method_demarshal_hello, 0, },
	[PW_CORE_METHOD_SYNC] = { &core_method_demarshal_sync, 0, },
	[PW_CORE_METHOD_PONG] = { &core_method_demarshal_pong, 0, },
	[PW_CORE_METHOD_ERROR] = { &core_method_demarshal_error, 0, },
	[PW_CORE_METHOD_GET_REGISTRY] = { &core_method_demarshal_get_registry, 0, },
	[PW_CORE_METHOD_CREATE_OBJECT] = { &core_method_demarshal_create_object, 0, },
	[PW_CORE_METHOD_DESTROY] = { &core_method_demarshal_destroy, 0, }
};

static const struct pw_core_events pw_protocol_native_core_event_marshal = {
	PW_VERSION_CORE_EVENTS,
	.info = &core_event_marshal_info,
	.done = &core_event_marshal_done,
	.ping = &core_event_marshal_ping,
	.error = &core_event_marshal_error,
	.remove_id = &core_event_marshal_remove_id,
	.bound_id = &core_event_marshal_bound_id,
	.add_mem = &core_event_marshal_add_mem,
	.remove_mem = &core_event_marshal_remove_mem,
	.bound_props = &core_event_marshal_bound_props,
};

static const struct pw_protocol_native_demarshal
pw_protocol_native_core_event_demarshal[PW_CORE_EVENT_NUM] =
{
	[PW_CORE_EVENT_INFO] = { &core_event_demarshal_info, 0, },
	[PW_CORE_EVENT_DONE] = { &core_event_demarshal_done, 0, },
	[PW_CORE_EVENT_PING] = { &core_event_demarshal_ping, 0, },
	[PW_CORE_EVENT_ERROR] = { &core_event_demarshal_error, 0, },
	[PW_CORE_EVENT_REMOVE_ID] = { &core_event_demarshal_remove_id, 0, },
	[PW_CORE_EVENT_BOUND_ID] = { &core_event_demarshal_bound_id, 0, },
	[PW_CORE_EVENT_ADD_MEM] = { &core_event_demarshal_add_mem, 0, },
	[PW_CORE_EVENT_REMOVE_MEM] = { &core_event_demarshal_remove_mem, 0, },
	[PW_CORE_EVENT_BOUND_PROPS] = { &core_event_demarshal_bound_props, 0, },
};

static const struct pw_protocol_marshal pw_protocol_native_core_marshal = {
	PW_TYPE_INTERFACE_Core,
	PW_VERSION_CORE,
	0,
	PW_CORE_METHOD_NUM,
	PW_CORE_EVENT_NUM,
	.client_marshal = &pw_protocol_native_core_method_marshal,
	.server_demarshal = pw_protocol_native_core_method_demarshal,
	.server_marshal = &pw_protocol_native_core_event_marshal,
	.client_demarshal = pw_protocol_native_core_event_demarshal,
};
```
