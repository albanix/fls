/**
	@code by albanix.
	This code is licensed under the LGPL.
**/

#include <stdio.h>
#include <dirent.h>
#include <string.h>

int main(int argc, char *argv[]) {
	DIR *dir = NULL;
	if(argc > 1) {
		dir = opendir(argv[1]);
	} else {
		dir = opendir(".");
	}

	if(dir == NULL) {
		perror("I can't read dir!");
		return 1;
	}

	struct dirent *entry;

	while((entry = readdir(dir)) != NULL) {
		const char* d_name = entry->d_name;
		if(strcmp(d_name, ".") == 0) continue;
		if(strcmp(d_name, "..") == 0) continue;

		printf("%s\n", d_name);
	}

	closedir(dir);
	return 0;
}
