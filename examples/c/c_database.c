#include <stdio.h>
#include <string.h>
#include <stdlib.h>
struct Record { int id; char name[32]; };
struct Record db[100];
int db_count = 0;
void db_insert(int id, const char *name) { db[db_count].id=id; strcpy(db[db_count].name,name); db_count++; }
struct Record *db_find(int id) { for(int i=0;i<db_count;i++) if(db[i].id==id) return &db[i]; return NULL; }
int main() { db_insert(1,"Alice"); db_insert(2,"Bob"); struct Record *r=db_find(1); if(r) printf("Found: %s\n",r->name); return 0; }