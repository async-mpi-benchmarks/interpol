mpiexec -n 40 -env LD_PRELOAD /home/sbstndbs/sharing/PPN/interpol/libinterpol.so -env LD_LIBRARY_PATH $LD_LIBRARY_PATH -env OMP_NUM_THREADS 1 --host threadripper,ryzen,asus --bind-to l3cache  ./lbm
