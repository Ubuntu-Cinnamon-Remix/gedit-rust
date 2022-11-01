// Generated by gir (https://github.com/gtk-rs/gir @ 81e2ef9e1ab7)
// from ../../gir-files (@ 3b6fe0a33676)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((guint) GEDIT_DEBUG_APP);
    PRINT_CONSTANT((guint) GEDIT_DEBUG_COMMANDS);
    PRINT_CONSTANT((guint) GEDIT_DEBUG_DOCUMENT);
    PRINT_CONSTANT((guint) GEDIT_DEBUG_METADATA);
    PRINT_CONSTANT((guint) GEDIT_DEBUG_PANEL);
    PRINT_CONSTANT((guint) GEDIT_DEBUG_PLUGINS);
    PRINT_CONSTANT((guint) GEDIT_DEBUG_PREFS);
    PRINT_CONSTANT((guint) GEDIT_DEBUG_TAB);
    PRINT_CONSTANT((guint) GEDIT_DEBUG_UTILS);
    PRINT_CONSTANT((guint) GEDIT_DEBUG_VIEW);
    PRINT_CONSTANT((guint) GEDIT_DEBUG_WINDOW);
    PRINT_CONSTANT((guint) GEDIT_NO_DEBUG);
    PRINT_CONSTANT((gint) GEDIT_TAB_NUM_OF_STATES);
    PRINT_CONSTANT((gint) GEDIT_TAB_STATE_CLOSING);
    PRINT_CONSTANT((gint) GEDIT_TAB_STATE_EXTERNALLY_MODIFIED_NOTIFICATION);
    PRINT_CONSTANT((gint) GEDIT_TAB_STATE_GENERIC_ERROR);
    PRINT_CONSTANT((gint) GEDIT_TAB_STATE_LOADING);
    PRINT_CONSTANT((gint) GEDIT_TAB_STATE_LOADING_ERROR);
    PRINT_CONSTANT((gint) GEDIT_TAB_STATE_NORMAL);
    PRINT_CONSTANT((gint) GEDIT_TAB_STATE_PRINTING);
    PRINT_CONSTANT((gint) GEDIT_TAB_STATE_REVERTING);
    PRINT_CONSTANT((gint) GEDIT_TAB_STATE_REVERTING_ERROR);
    PRINT_CONSTANT((gint) GEDIT_TAB_STATE_SAVING);
    PRINT_CONSTANT((gint) GEDIT_TAB_STATE_SAVING_ERROR);
    PRINT_CONSTANT((gint) GEDIT_TAB_STATE_SHOWING_PRINT_PREVIEW);
    PRINT_CONSTANT((guint) GEDIT_WINDOW_STATE_ERROR);
    PRINT_CONSTANT((guint) GEDIT_WINDOW_STATE_LOADING);
    PRINT_CONSTANT((guint) GEDIT_WINDOW_STATE_NORMAL);
    PRINT_CONSTANT((guint) GEDIT_WINDOW_STATE_PRINTING);
    PRINT_CONSTANT((guint) GEDIT_WINDOW_STATE_SAVING);
    return 0;
}
