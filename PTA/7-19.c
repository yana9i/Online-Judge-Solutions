#include <stdio.h>
#include <ctype.h>

int main()
{
    char str[81];
    int offset;
    gets(str);
    scanf("%d", &offset);
    offset %= 26;
    for (int i = 0;str[i] != '\0';i++) {
        if (isalpha(str[i])) {
            if (islower(str[i]))
                str[i] = ('a' + (str[i] - 'a' + offset + 26) % 26);
            if (isupper(str[i]))
                str[i] = ('A' + (str[i] - 'A' + offset + 26) % 26);
        }
        printf("%c", str[i]);
    }

}