#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LEN_BUFFER 100


int get_int_len (int value){
  int l=1;
  while(value>9){ l++; value/=10; }
  return l;
}





int main(int argc, char **argv){



  char str_input[LEN_BUFFER];
  char str_output[LEN_BUFFER];  
  char str_sharp[2] = "#" ;  

  printf("This is a code to resync the .json traces ...\n") ; 
  printf("Please select the name of the .json files : \n");
  printf("Please use * to specify the rank number position\n");
  scanf("%s", str_input) ; 
  

  int i = 0 ;    // count the position in the filename 
  int position = -1 ;   // set the * postion in the filename
  strcpy(str_output, str_input) ; 
  int occ = 0 ; // define the size of the input filename

  while(i < LEN_BUFFER + 1){
    if ( strncmp(str_input+i, "*", 1) == 0){
      strncpy(str_output+i, str_sharp,  1) ; 
      if (position == -1 ){
        position = i ; 
      }
      else{
           printf("Please specify a valid failename ...\n");
           return 0;
      }
    }
    i++;
    occ++;
  }

  if (position < 0){
    printf("Please set a correct file name ... \n\n");
    return 0 ; 
  }
  else{
    printf("You set %s.\n", str_output) ; 
  }


  int number_of_ranks = 0 ; 
  printf("Please enter the number of ranks : \n");
  scanf("%d", &number_of_ranks) ;
  printf("You set %d ranks.\n", number_of_ranks) ; 

  if (number_of_ranks <= 0 ){
    printf("Please specify 1 rank or more \n");
    return 0 ; 
  }

// malloc of file 

FILE* files = malloc( number_of_ranks * sizeof(FILE*) ) ; 
char **filename = malloc( number_of_ranks * sizeof(char**)) ; 
for (int i = 0 ; i < number_of_ranks; i++){
  filename[i] = (char*)malloc( LEN_BUFFER * sizeof(char*)) ; 
}


char str_number[10];
   
for (int i = 0 ; i < number_of_ranks; i++){
  // modifcation of the filename with rank number
  sprintf(str_number, "%d", i) ; 
  int size_number = get_int_len(i) ; 
  strncpy(filename[i], str_output, position ) ; 
  strncpy(filename[i] + position, str_number, size_number) ; 
  strncpy(filename[i] + position + size_number, str_input + position + 1 , occ -1  - position) ; 
  printf("File : %s\n", filename[i]) ; 
  

  // 





}


for (i = 0 ; i < number_of_ranks ; i++){
  free(filename[i]) ; 
} 
free(filename) ; 
free(files) ; 


  return 0 ; 
}
