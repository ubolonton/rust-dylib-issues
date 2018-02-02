#include <dlfcn.h>
#include <stdio.h>
#include <unistd.h>

#define LIB_PATH "../app/target/debug/libapp.dylib"

int main(int argc, char *argv[])
{
	while (1) {
		sleep(1);

		void *handle = dlopen(LIB_PATH, RTLD_NOW);
		if (!handle) {
			printf("Failed to open library.\n");
			return 1;
		}

		char *(*get_message)(void) = dlsym(handle, "get_message");
		if (!get_message) {
			printf("Failed to retrieve get_message symbol: %s\n", dlerror());
			return 1;
		}

		printf("Message: %s\n", get_message());

		if (dlclose(handle)) {
			printf("Failed to close handle: %s\n", dlerror());
		}
	}
}
