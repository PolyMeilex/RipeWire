# Methods
- PW_CLIENT_METHOD_ADD_LISTENER 0
- [PW_CLIENT_METHOD_ERROR 1](#error)
- [PW_CLIENT_METHOD_UPDATE_PROPERTIES 2](#update-properties)
- [PW_CLIENT_METHOD_GET_PERMISSIONS 3](#get-permissions)
- [PW_CLIENT_METHOD_UPDATE_PERMISSIONS 4](#update-permissions)
- PW_CLIENT_METHOD_NUM 5

## Error
Send an error to a client

\param id the global id to report the error on
\param res an errno style error code
\param message an error string

This requires W and X permissions on the client.

```c
static int client_marshal_error(void *object, uint32_t id, int res, const char *error)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_proxy(proxy, PW_CLIENT_METHOD_ERROR, NULL);
	spa_pod_builder_add_struct(b,
			       SPA_POD_Int(id),
			       SPA_POD_Int(res),
			       SPA_POD_String(error));
	return pw_protocol_native_end_proxy(proxy, b);
}
```

```c
static int client_demarshal_error(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct spa_pod_parser prs;
	uint32_t id, res;
	const char *error;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Int(&id),
				SPA_POD_Int(&res),
				SPA_POD_String(&error)) < 0)
		return -EINVAL;

	return pw_resource_notify(resource, struct pw_client_methods, error, 0, id, res, error);
}
```

## Update properties
Update client properties

\param props new properties

This requires W and X permissions on the client.

```c
static int client_marshal_update_properties(void *object, const struct spa_dict *props)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;
	struct spa_pod_frame f;

	b = pw_protocol_native_begin_proxy(proxy, PW_CLIENT_METHOD_UPDATE_PROPERTIES, NULL);

	spa_pod_builder_push_struct(b, &f);
	push_dict(b, props);
	spa_pod_builder_pop(b, &f);

	return pw_protocol_native_end_proxy(proxy, b);
}
```

```c
static int client_demarshal_update_properties(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct spa_dict props = SPA_DICT_INIT(NULL, 0);
	struct spa_pod_parser prs;
	struct spa_pod_frame f[2];

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_push_struct(&prs, &f[0]) < 0)
		return -EINVAL;

	parse_dict_struct(&prs, &f[1], &props);

	return pw_resource_notify(resource, struct pw_client_methods, update_properties, 0,
			&props);
}
```

## Get permissions
Get client permissions

A permissions event will be emitted with the permissions.

\param index the first index to query, 0 for first
\param num the maximum number of items to get

This requires W and X permissions on the client.

```c
static int client_marshal_get_permissions(void *object, uint32_t index, uint32_t num)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;

	b = pw_protocol_native_begin_proxy(proxy, PW_CLIENT_METHOD_GET_PERMISSIONS, NULL);

	spa_pod_builder_add_struct(b,
			SPA_POD_Int(index),
			SPA_POD_Int(num));

	return pw_protocol_native_end_proxy(proxy, b);
}
```

```c
static int client_demarshal_get_permissions(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct spa_pod_parser prs;
	uint32_t index, num;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_get_struct(&prs,
				SPA_POD_Int(&index),
				SPA_POD_Int(&num)) < 0)
		return -EINVAL;

	return pw_resource_notify(resource, struct pw_client_methods, get_permissions, 0, index, num);
}
```

## Update permissions
Manage the permissions of the global objects for this
client

Update the permissions of the global objects using the
provided array with permissions

Globals can use the default permissions or can have specific
permissions assigned to them.

\param n_permissions number of permissions
\param permissions array of permissions

This requires W and X permissions on the client.

```c
static int client_marshal_update_permissions(void *object, uint32_t n_permissions,
		const struct pw_permission *permissions)
{
	struct pw_proxy *proxy = object;
	struct spa_pod_builder *b;
	struct spa_pod_frame f;
	uint32_t i;

	b = pw_protocol_native_begin_proxy(proxy, PW_CLIENT_METHOD_UPDATE_PERMISSIONS, NULL);

	spa_pod_builder_push_struct(b, &f);
	spa_pod_builder_int(b, n_permissions);
	for (i = 0; i < n_permissions; i++) {
		spa_pod_builder_int(b, permissions[i].id);
		spa_pod_builder_int(b, permissions[i].permissions);
	}
	spa_pod_builder_pop(b, &f);

	return pw_protocol_native_end_proxy(proxy, b);
}
```

```c
static int client_demarshal_update_permissions(void *object, const struct pw_protocol_native_message *msg)
{
	struct pw_resource *resource = object;
	struct pw_permission *permissions;
	struct spa_pod_parser prs;
	struct spa_pod_frame f[1];
	uint32_t n_permissions;

	spa_pod_parser_init(&prs, msg->data, msg->size);

	parse_permissions_struct(&prs, &f[0], n_permissions, permissions);

	return pw_resource_notify(resource, struct pw_client_methods, update_permissions, 0,
			n_permissions, permissions);
}
```

# Events
- [PW_CLIENT_EVENT_INFO 0](#info)
- [PW_CLIENT_EVENT_PERMISSIONS 1](#permissions)
- PW_CLIENT_EVENT_NUM 2

## Info
Notify client info

\param info info about the client

```c
static void client_marshal_info(void *data, const struct pw_client_info *info)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;
	struct spa_pod_frame f;

	b = pw_protocol_native_begin_resource(resource, PW_CLIENT_EVENT_INFO, NULL);

	spa_pod_builder_push_struct(b, &f);
	spa_pod_builder_add(b,
			    SPA_POD_Int(info->id),
			    SPA_POD_Long(info->change_mask),
			    NULL);
	push_dict(b, info->change_mask & PW_CLIENT_CHANGE_MASK_PROPS ? info->props : NULL);
	spa_pod_builder_pop(b, &f);

	pw_protocol_native_end_resource(resource, b);
}
```

```c
static int client_demarshal_info(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct spa_pod_parser prs;
	struct spa_pod_frame f[2];
	struct spa_dict props = SPA_DICT_INIT(NULL, 0);
	struct pw_client_info info = { .props = &props };

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_push_struct(&prs, &f[0]) < 0 ||
	    spa_pod_parser_get(&prs,
			SPA_POD_Int(&info.id),
			SPA_POD_Long(&info.change_mask), NULL) < 0)
		return -EINVAL;

	parse_dict_struct(&prs, &f[1], &props);

	return pw_proxy_notify(proxy, struct pw_client_events, info, 0, &info);
}
```

## Permissions
Notify a client permission

Event emitted as a result of the get_permissions method.

\param default_permissions the default permissions
\param index the index of the first permission entry
\param n_permissions the number of permissions
\param permissions the permissions
```c
static void client_marshal_permissions(void *data, uint32_t index, uint32_t n_permissions,
		const struct pw_permission *permissions)
{
	struct pw_resource *resource = data;
	struct spa_pod_builder *b;
	struct spa_pod_frame f[2];
	uint32_t i, n = 0;

	b = pw_protocol_native_begin_resource(resource, PW_CLIENT_EVENT_PERMISSIONS, NULL);

	for (i = 0; i < n_permissions; i++) {
		if (permissions[i].permissions != PW_PERM_INVALID)
			n++;
	}

	spa_pod_builder_push_struct(b, &f[0]);
	spa_pod_builder_int(b, index);
	spa_pod_builder_push_struct(b, &f[1]);
	spa_pod_builder_int(b, n);

	for (i = 0; i < n_permissions; i++) {
		if (permissions[i].permissions == PW_PERM_INVALID)
			continue;
		spa_pod_builder_int(b, permissions[i].id);
		spa_pod_builder_int(b, permissions[i].permissions);
	}
	spa_pod_builder_pop(b, &f[1]);
	spa_pod_builder_pop(b, &f[0]);

	pw_protocol_native_end_resource(resource, b);
}
```

```c
static int client_demarshal_permissions(void *data, const struct pw_protocol_native_message *msg)
{
	struct pw_proxy *proxy = data;
	struct pw_permission *permissions;
	struct spa_pod_parser prs;
	struct spa_pod_frame f[2];
	uint32_t index, n_permissions;

	spa_pod_parser_init(&prs, msg->data, msg->size);
	if (spa_pod_parser_push_struct(&prs, &f[0]) < 0 ||
	    spa_pod_parser_get(&prs,
		    SPA_POD_Int(&index), NULL) < 0)
		return -EINVAL;

	parse_permissions_struct(&prs, &f[1], n_permissions, permissions);

	return pw_proxy_notify(proxy, struct pw_client_events, permissions, 0, index, n_permissions, permissions);
}
```

