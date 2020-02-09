#include <stdio.h>
#include <gtk/gtk.h>

#ifndef _FDIALOG_GTK
#define _FDIALOG_GTK

char* open_dialog()
{
	gtk_init(NULL, NULL);

	char* filename = NULL;
	GtkWidget* dialog = NULL;
	GtkFileFilter* filter;

	dialog = gtk_file_chooser_dialog_new("Open File",
		NULL,
		GTK_FILE_CHOOSER_ACTION_OPEN,
		"Cancel", GTK_RESPONSE_CANCEL,
		"Open", GTK_RESPONSE_ACCEPT,
		NULL);
	filter = gtk_file_filter_new();

	gtk_file_filter_set_name(filter, "Filter name");
	gtk_file_filter_add_pattern(filter, "*.rs");
//	gtk_file_filter_add_pattern(filter, "*.toml");
	gtk_file_chooser_add_filter(GTK_FILE_CHOOSER(dialog), filter);

	if (gtk_dialog_run(GTK_DIALOG(dialog)) == GTK_RESPONSE_ACCEPT)
	{
		filename = gtk_file_chooser_get_filename(GTK_FILE_CHOOSER(dialog));
	}
	else
	{

	}

	gtk_widget_destroy(dialog);

	return filename;
}

void free_string(char* string)
{
	free(string);
}

#endif