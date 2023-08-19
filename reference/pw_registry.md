# Methods
- PW_REGISTRY_METHOD_ADD_LISTENER 0
- [PW_REGISTRY_METHOD_BIND 1](#bind)
- [PW_REGISTRY_METHOD_DESTROY 2](#destroy)
- PW_REGISTRY_METHOD_NUM 3

## Bind
Bind to a global object

Bind to the global object with \a id and use the client proxy
with new_id as the proxy. After this call, methods can be
send to the remote global object and events can be received

\param id the global id to bind to
\param type the interface type to bind to
\param version the interface version to use
\returns the new object

```c
static void * registry_marshal_bind(void *object, uint32_t id,
				  const char *type, uint32_t version, size_t user_data_size)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;
	struct pw_proxy *res;
	uint32_t new_id;

	res = pw_proxy_new(object, type, version, user_data_size);
	if (res == NULL)
		return NULL;

	new_id = pw_proxy_get_id(res);

	b = pw_protocol_native_begin_proxy(proxy, PW_REGISTRY_METHOD_BIND, NULL);

	spa_pod_builder_add_struct(b,
			       SPA_POD_Int(id),
			       SPA_POD_String(type),
			       SPA_POD_Int(version),
			       SPA_POD_Int(new_id));

	pw_protocol_native_end_proxy(proxy, b);

	return (void *) res;
}
```

```c
static int registry_demarshal_bind(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct spa_pod_parser prs;
	uint32_t id, version, new_id;
	char *type;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
			SPA_POD_Int(&id),
			SPA_POD_String(&type),
			SPA_POD_Int(&version),
			SPA_POD_Int(&new_id)) < 0)
		return -EINVAL;

	return pw_resource_notify(resource, struct pw_registry_methods, bind, 0, id, type, version, new_id);
}
```

## Destroy
Attempt to destroy a global object

Try to destroy the global object.

\param id the global id to destroy. The client needs X permissions
on the global.

```c
static int registry_marshal_destroy(void *object, uint32_t id)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_proxy(proxy, PW_REGISTRY_METHOD_DESTROY, NULL);
	spa_pod_builder_add_struct(b,
			       SPA_POD_Int(id));
	return pw_protocol_native_end_proxy(proxy, b);
}
```

```c
static int registry_demarshal_destroy(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct spa_pod_parser prs;
	uint32_t id;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
			SPA_POD_Int(&id)) < 0)
		return -EINVAL;

	return pw_resource_notify(resource, struct pw_registry_methods, destroy, 0, id);
}
```

# Events
- [PW_REGISTRY_EVENT_GLOBAL 0](#global)
- [PW_REGISTRY_EVENT_GLOBAL_REMOVE 1](#global-remove)
- PW_REGISTRY_EVENT_NUM 2

## Global
Notify of a new global object

The registry emits this event when a new global object is
available.

\param id the global object id
\param permissions the permissions of the object
\param type the type of the interface
\param version the version of the interface
\param props extra properties of the global

```c
static void registry_marshal_global(void *data, uint32_t id, uint32_t permissions,
				    const char *type, uint32_t version, const struct spa_dict *props)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;
	struct spa_pod_frame f;

	b = pw_protocol_native_begin_resource(resource, PW_REGISTRY_EVENT_GLOBAL, NULL);

	spa_pod_builder_push_struct(b, &f);
	spa_pod_builder_add(b,
			    SPA_POD_Int(id),
			    SPA_POD_Int(permissions),
			    SPA_POD_String(type),
			    SPA_POD_Int(version),
			    NULL);
	push_dict(b, props);
	spa_pod_builder_pop(b, &f);

	pw_protocol_native_end_resource(resource, b);
}
```

```c
static int registry_demarshal_global(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_pod_parser prs;
	struct spa_pod_frame f[2];
	uint32_t id, permissions, version;
	char *type;
	struct spa_dict props = SPA_DICT_INIT(NULL, 0);

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_push_struct(&prs, &f[0]) < 0 ||
	    spa_pod_parser_get(&prs,
			SPA_POD_Int(&id),
			SPA_POD_Int(&permissions),
			SPA_POD_String(&type),
			SPA_POD_Int(&version), NULL) < 0)
		return -EINVAL;

	parse_dict_struct(&prs, &f[1], &props);

	return pw_proxy_notify(proxy, struct pw_registry_events,
			global, 0, id, permissions, type, version, &props);
}
```

## Global remove
Notify of a global object removal

Emitted when a global object was removed from the registry.
If the client has any bindings to the global, it should destroy
those.

\param id the id of the global that was removed
```c
static void registry_marshal_global_remove(void *data, uint32_t id)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_resource(resource, PW_REGISTRY_EVENT_GLOBAL_REMOVE, NULL);

	spa_pod_builder_add_struct(b, SPA_POD_Int(id));

	pw_protocol_native_end_resource(resource, b);
}
```

```c
static int registry_demarshal_global_remove(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_pod_parser prs;
	uint32_t id;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Int(&id)) < 0)
		return -EINVAL;

	return pw_proxy_notify(proxy, struct pw_registry_events, global_remove, 0, id);
}
```

