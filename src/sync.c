#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LEN_BUFFER 100


int main(int argc, char **argv){



  char str_input[LEN_BUFFER];
  char str_output[LEN_BUFFER];  
  char str_sharp[2] = "#" ;  

  printf("This is a code to resync the .json traces ...\n") ; 
  printf("Please select the name of the .json files : \n");
  printf("Please use * to specify the rank number position\n");
  scanf("%s", str_input) ; 
  

  int i = 0 ; 
  int occ = 0 ;
  int last_position = -1 ; 
  strcpy(str_output, str_input) ; 
  while(i < LEN_BUFFER + 1){
    if ( strncmp(str_input+i, "*", 1) == 0){
      strncpy(str_output+i, str_sharp,  1) ; 
      if (occ == 0 ){
        last_position = i ;
      }
      else{
         if ((i - last_position) != 1 ){
           printf("Please specify a valid failename ...\n");
           return 0 ; 
         }
         else{
         // we need to add the indexes of the numbers 
         }
      }
      occ++ ; 
    }
    i++;
  }

  if (occ < 1){
    printf("Please set a correct file name ... \n\n");
  }
  else{
    printf("You set %s.\n", str_output) ; 
  }
    


  return 0 ; 
}
