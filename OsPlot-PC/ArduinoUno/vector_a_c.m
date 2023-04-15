function str = vector_a_c(V)
  str = sprintf("uint8_t mostres[] = {");
  for elemV = V(1:end-1)
      str = [str sprintf("%d,", elemV)];
  end
  str = [str sprintf("%d};\n", V(end))];
  printf(str);
endfunction