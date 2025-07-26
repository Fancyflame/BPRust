struct Handler
{
    void *context;
    void *fframe;
    void *z_param_result;
};

struct CppFunctionTable
{
    void (*handle_custom_thunk)(Handler *handler,
                                void *user_data,
                                void (*resolve_param)(void *user_data, Handler *handler),
                                void (*call_function)(void *user_data, void *u_object));
    void (*process_event)(void *u_object, const char *fn_name, void *params);
};

extern "C"
{

    void BPRustSys_init(CppFunctionTable table);

} // extern "C"
