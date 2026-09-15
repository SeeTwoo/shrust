#include <fcntl.h>
#include <stdio.h>
#include <unistd.h>

#ifndef BUFFER_SIZE
# define BUFFER_SIZE 64
#endif

void	display_file(char const *filename)
{
	int	fd = open(filename, O_RDONLY);
	
	if (fd == -1) {
		dprintf(2, "ft_cat: %s: no such file or directory\n", filename);
		return ;
	}
	while (1) {
		char	buffer[BUFFER_SIZE];
		ssize_t	bytes_read = read(fd, buffer, BUFFER_SIZE);

		if (bytes_read <= 0)
			return ;
		write(1, buffer, bytes_read);
	}
}

int	main(int ac, char **av)
{
	if (ac == 1) {
		dprintf(2, "ft_cat: no argument provided\n");
	}

	for (char **args = &av[1]; *args; args++) {
		display_file(*args);
	}
}
