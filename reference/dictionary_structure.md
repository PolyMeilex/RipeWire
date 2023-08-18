# Dictionary related stuff

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
