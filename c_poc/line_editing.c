#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#ifndef BUFFER_SIZE
#define BUFFER_SIZE 32
#endif

char	*line_editor()
{
	char	buffer[BUFFER_SIZE];
	size_t	size = 0;

	write(2, " > ", 3);
	while (1) {
		if (size == BUFFER_SIZE)
			break ;
		char	c;
		ssize_t	byte_read = read(0, &c, 1);

		if (byte_read <= 0)
			return NULL;
		if (c == '\n')
			break ;
		buffer[size] = c;
		size++;
	}
	return strndup(buffer, size);
}

int	main()
{
	char	*line = NULL;

	while (1) {
		line = line_editor();
		
		if (!line)
			return 1;
		printf("%s\n", line);
		if (strcmp(line, "exit") == 0)
			break ;
		free(line);
	}
	free(line);
	return 0;
}
