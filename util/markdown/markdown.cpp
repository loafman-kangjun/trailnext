#include <cstring>
#include "markdown.h"
#include <cmark.h>

namespace util_markdown {
    int test() {
        const char *markdown = "# Welcome";
        char *html = cmark_markdown_to_html(markdown, strlen(markdown), CMARK_OPT_DEFAULT);
        printf(html);
    }
}