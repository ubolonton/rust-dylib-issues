#include <dlfcn.h>
#include <stdio.h>
#include <unistd.h>

#define LIB_PATH "../module/target/debug/libmodule.dylib"
/* #define LIB_PATH "../module/target/libmodule-c.so" */

int main(int argc, char *argv[])
{
	while (1) {
		sleep(1);

		void *handle = dlopen(LIB_PATH, RTLD_NOW);
		if (!handle) {
			printf("Failed to open library.\n");
			return 1;
		}

		void *(*init)(void) = dlsym(handle, "init");
		if (!init) {
			printf("Failed to retrieve init symbol: %s\n", dlerror());
			return 1;
		}

		init();

		if (dlclose(handle)) {
			printf("Failed to close handle: %s\n", dlerror());
		}
	}
}
