#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <json.h>

#define LEN_BUFFER 100


int get_int_len (int value){
  int l=1;
  while(value>9){ l++; value/=10; }
  return l;
}


struct struct_input{
  char str_input[LEN_BUFFER] ;
  char str_output[LEN_BUFFER];
  char str_sharp[2];
  int position ;
  int occ;
  int number_of_ranks ;
  //FILE** files ; 
  //char **filename;
};

struct struct_fields{

  FILE** files ;
  json_object **json_files ; 
  char** filename;
  unsigned long long *length;

  unsigned long long * init_tsc ; 
  unsigned long long * end_tsc ; 
  unsigned long long *diff;
  double * ratio ; 
};



int input(struct struct_input* sinput){

  printf("This is a code to resync the .json traces ...\n") ; 
  printf("Please select the name of the .json files : \n");
  printf("Please use * to specify the rank number position\n");
  scanf("%s", sinput->str_input) ;

  int i = 0 ;    // count the position in the filename 
  sinput->position = -1 ;   // set the * postion in the filename
  strcpy(sinput->str_output, sinput->str_input) ; 
  sinput->occ = 0 ; // define the size of the input filename

  while(i < LEN_BUFFER + 1){
    if ( strncmp(sinput->str_input+i, "*", 1) == 0){
      strncpy(sinput->str_output+i, sinput->str_sharp,  1) ; 
      if (sinput->position == -1 ){
        sinput->position = i ; 
      }
      else{
           printf("Please specify a valid failename ...\n");
           return 0;
      }
    }
    i++;
    sinput->occ++;
  }

  if (sinput->position < 0){
    printf("Please set a correct file name ... \n\n");
    return 0 ; 
  }
  else{
    printf("You set %s.\n", sinput->str_output) ; 
  }

  sinput->number_of_ranks = 0 ; 
  printf("Please enter the number of ranks : \n");
  scanf("%d", &sinput->number_of_ranks) ;
  printf("You set %d ranks.\n", sinput->number_of_ranks);
  if (sinput->number_of_ranks <= 0 ){
    printf("Please specify 1 rank or more \n");
    return 0 ; 
  }
  return 1 ; 
}



int malloc_fields(struct struct_input* sinput, struct struct_fields* fields){
  int number_of_ranks = sinput->number_of_ranks ;


  // for files
  fields->files = malloc(number_of_ranks * sizeof(FILE*));
  fields->json_files = malloc(number_of_ranks * sizeof(json_object*));
  fields->filename = malloc(number_of_ranks * sizeof(char**)) ; 
  for (int i  =0 ; i < number_of_ranks ; i++){
    fields->filename[i] = malloc(LEN_BUFFER * sizeof(char*)) ; 
  }
  fields->length = malloc( number_of_ranks * sizeof(unsigned long long));
  
  // for counter
  fields->init_tsc = malloc( number_of_ranks * sizeof(unsigned long long));
  fields->end_tsc = malloc( number_of_ranks * sizeof(unsigned long long)) ; 
  fields->diff = malloc (number_of_ranks * sizeof(unsigned long long)) ; 
  fields->ratio = malloc( number_of_ranks * sizeof(double)) ;

  return 1 ; 
}


int load_files(struct struct_input * sinput, struct struct_fields * sfields){

  char str_number[10];
   
  for (int i = 0 ; i < sinput->number_of_ranks; i++){
    // modifcation of the filename with rank number
    sprintf(str_number, "%d", i) ; 
    int size_number = get_int_len(i) ; 
    strncpy(sfields->filename[i], sinput->str_output, sinput->position ) ; 
    strncpy(sfields->filename[i] + sinput->position, str_number, size_number) ; 
    strncpy(sfields->filename[i] + sinput->position + size_number, sinput->str_input + sinput->position + 1 , sinput->occ -1  - sinput->position) ; 
    printf("File : %s\n", sfields->filename[i]) ; 
  }
  
  int status = 1 ; 
  for (int i = 0 ; i < sinput->number_of_ranks ; i++){
    //sfields->files[i] = fopen(sfields->filename[i], "rw") ; 
    sfields->json_files[i] = json_object_from_file(sfields->filename[i]) ; 
    //if (sfields->files[i] == NULL){
    //  status = 0 ; 
    //  printf("Can't load file %d ... \n", i) ; 
    //}
  }
  printf("Files loaded \n");

  return 1 ; 
}



void find_tsc_values(struct struct_input *sinput, struct struct_fields *sfields, char * buffer){
 
  for (int i = 0 ; i < sinput->number_of_ranks ; i++){
    struct json_object *parsed_json ;

    //fread(buffer, 102400, 1, sfields->files[i]);
    //parsed_json = json_tokener_parse(buffer) ;
    //sfields->length[i] = (int)json_object_array_length(parsed_json) ;
    sfields->length[i] = (int)json_object_array_length(sfields->json_files[i]);

//    load_json(sinput, sfields, parsed_json, buffer, i) ; 

    int length = sfields->length[i] ;
    printf("there is %d length\n", length) ; 
    int state_init = 0 ; 
    int state_end = 0 ; 
    for (int j = 0 ; j < length ; j++){
      //json_object *element = json_object_array_get_idx(parsed_json, j);
      json_object * element  = json_object_array_get_idx(sfields->json_files[i],j);
      json_object * types = json_object_object_get(element, "type");
      json_object * tscs = json_object_object_get(element, "tsc");

      char *types_str = json_object_get_string(types);
      char *tscs_str = json_object_get_string(tscs);

      printf("type : %s\n", types_str);

      if ((strcmp(types_str, "MpiInit") == 0) || (strcmp(types_str, "MpiInitThread") == 0)){
        sfields->init_tsc[i] = strtoull (tscs_str, NULL, 10);
         printf("The init_tsc value is %llu\n", sfields->init_tsc[i]);
         state_init = 1 ; 
      }
      if (strcmp(types_str, "MpiFinalize")==0){
        sfields->end_tsc[i] = strtoull(tscs_str, NULL, 10) ; 
        printf("The end_tsc value is %llu\n", sfields->end_tsc[i]) ; 
        state_end = 1 ; 
      }

      if ((state_init == 1) && (state_end == 1)){
        sfields->diff[i] = - sfields->init_tsc[i] + sfields->end_tsc[i] ;
        state_init = 0 ; 
        state_end = 0 ; 
      }   
      else{
        printf("Cannot find tscs...\n");
      }
    }
  }
}

void compute_ratio(struct struct_input *sinput, struct struct_fields *sfields){

  // now we compute a ratio 
  unsigned long long diff = sfields->diff[0] ; 
  sfields->ratio[0] = 1.0 ; 
  for(int i = 1 ; i < sinput->number_of_ranks; i++){
    sfields->ratio[i] = (double)( (double)diff/ (double)sfields->diff[i]);
    printf("ratio : %f\n", sfields->ratio[i]) ; 
    printf("from value %llu to value %llu\n" , sfields->end_tsc[i] , (unsigned long long)((double)sfields->end_tsc[i] * sfields->ratio[i]) - sfields->init_tsc[i]) ;
  }
}


void ajust_tsc(struct struct_input* sinput, struct struct_fields* sfields){

  for (int i = 0 ; i < sinput->number_of_ranks ; i++){
    struct json_object *parsed_json ;

    //fread(buffer, 102400, 1, sfields->files[i]);
    //parsed_json = json_tokener_parse(buffer) ;
    sfields->length[i] = (int)json_object_array_length(sfields->json_files[i]);
    //sfields->length[i] = (int)json_object_array_length(parsed_json) ;

//    load_json(sinput, sfields, parsed_json, buffer, i) ; 

    int length = sfields->length[i] ;
    printf("there is %d length\n", length) ; 
    int state_init = 0 ; 
    int state_end = 0 ; 
    for (int j = 0 ; j < length ; j++){
      json_object *element = json_object_array_get_idx(sfields->json_files[i], j);
      //json_object * types = json_object_object_get(element, "type");
      json_object * tscs = json_object_object_get(element, "tsc");

      //char *types_str = json_object_get_string(types);
      char *tscs_str = json_object_get_string(tscs);
      unsigned long long value = strtoull(tscs_str, NULL, 10) ;
      char value_str[12] ; 
      sprintf(value_str, "%llu", value) ;
      value = (unsigned long long)(value * sfields->ratio[i] - sfields->init_tsc[i]) ; 
      //json_object_set_string(tscs, value_str)  ;
      printf("puttet value %llu\n", value) ; 
      int status_set = json_object_set_int64(tscs, value);
      if (status_set == 0){
        printf("issue : int64\n");
      }
      char * verification = json_object_get_string(tscs) ; 
      printf("verification : %s\n", verification) ; 


    }
  }
}



void put_json(struct struct_input* sinput, struct struct_fields * sfields){

  for (int i= 0 ; i < sinput->number_of_ranks ; i++){
    json_object_to_file(sfields->filename[i], sfields->json_files[i]);
    json_object_put(sfields->json_files[i]) ;
  }

}



void desalloc_fields(struct struct_fields *sfields, int number_of_ranks){
  free(sfields->files) ; 
  free(sfields->json_files);
  for (int i = 0 ; i < number_of_ranks ; i++){
    free(sfields->filename[i]) ; 
  }
  free(sfields->filename) ; 
  free(sfields->length) ; 
  free(sfields->init_tsc) ; 
  free(sfields-> end_tsc) ; 
  free(sfields->diff) ; 
  free(sfields->ratio) ; 
}

void desalloc_input(struct struct_input *sinput){
  
}

void desalloc(struct struct_input *sinput, struct struct_fields *sfields){
  desalloc_fields(sfields, sinput->number_of_ranks) ;
  desalloc_input(sinput) ; 
}


int main(int argc, char **argv){

  struct struct_input sinput  ;
  sinput.str_sharp[0]="#" ; 
  int status_input = input(&sinput) ;
  
  struct struct_fields sfields ; 
  int status_fields = malloc_fields(&sinput, &sfields) ;

  int status_load_files = load_files(&sinput, &sfields) ; 

  char buffer[102400] ; 
  find_tsc_values(&sinput, &sfields, buffer) ;
  compute_ratio(&sinput, &sfields) ; 
  ajust_tsc(&sinput, &sfields) ;
  put_json(&sinput, &sfields) ;  




  desalloc(&sinput, &sfields) ; 
  return 0 ; 
}
