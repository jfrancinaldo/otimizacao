import mysql.connector
import time
from timeit import default_timer as timer

import multiprocessing as mp
print("Numero de processadores: ", mp.cpu_count())


#import sys
#print(sys.argv[1])

st = time.time()
st_cpu = time.process_time()

cont_armado = 0
def check_armado(x):
    global cont_armado
    if x == 19:
        cont_armado +=1 

    return cont_armado

ganhadores = []
def check_ganhador(x):

    ganhadores.append(x) 

    return ganhadores
timeini_conn = time.time()

con = mysql.connector.connect(
  host="localhost",
  database="sorteio",
  user="conexaoSorteio",
  password="123456789"
)

if con.is_connected():

    sql_select_Query = "SELECT * FROM apostas"
    cursor = con.cursor()
    cursor.execute(sql_select_Query)
    # get all records
    records = cursor.fetchall()
    print("Total linhas da tabela: ", cursor.rowcount,"\n")

    
    saidaconn =  time.time() - timeini_conn
    print('Execution conn:', saidaconn, 'seconds')

    dados_input = ['05','59','46','43','33','56','11','39','25','21','12','23','47','35','53','31','09','48','03','02','30','28','38','36','40','27','34','24','42','22','58','20','32','17','51','55']

    print("Tamanho da entrada: ", len(dados_input),"\n")

    # list2 = [1, 3, 5]
    # intersection_set = set.intersection(set(list1), set(list2))
    # intersection_list = list(intersection_set)

    # print(intersection_list)
    total_ganhadores = []
    start = timer()
    start_for = time.time()
    for row in records:
    #    print("id = ", row[0])
    #    print("idtitulo = ", row[1])
    #    print("lote  = ", row[2])
    #    print("distribuidor  = ", row[3])
    #    print("dezenas  = ", row[4])
        win = len(list(set(row[4].split(',')) & set(dados_input)))
    
        total_armado = check_armado(win)
    #    print("Row1",row[1])
        try:
            if win == 20:
                total_ganhadores = check_ganhador(row[1])  
            else:
                total_ganhadores=0
        except:
           # print("Nenhum Ganhador")
            total_ganhadores = 0
            continue

            
        #  print (win)
        #  print("idtitulo ",row[1]," = ", len(list(set(row[4].split(',')) & set(dados_input))), "\n")
        # print("origem  = ", len(list(set(row[4].split(',')) & set(dados_input))), "\n")
    end = timer()
    print('Execution do FOR:', (time.time()- start_for), 'seconds')
    
    #se quantidade for 19 print (quantidade com 19)
    #se existir 1 ou mais com 20 retorna o array com os titulos correspondentes       

print("cont_armado ",total_armado) #saida esperada 34
print("array titulos ganhadores ",ganhadores,"\n")   #saida esperada 888424      


cpu = time.process_time()
res = cpu - st_cpu
print('CPU Execution time:', res, 'seconds')

if con.is_connected():
    cursor.close()
    con.close()
    print ("conexao encerrada")