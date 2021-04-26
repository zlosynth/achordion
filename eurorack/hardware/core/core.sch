EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L MCU_ST_STM32F4:STM32F427VGTx U1
U 1 1 60870DB2
P 2475 4025
F 0 "U1" H 2475 1136 50  0000 C CNN
F 1 " STM32F427VGT6" H 2475 1045 50  0000 C CNN
F 2 "Package_QFP:LQFP-100_14x14mm_P0.5mm" H 1775 1425 50  0001 R CNN
F 3 "http://www.st.com/st-web-ui/static/active/en/resource/technical/document/datasheet/DM00071990.pdf" H 2475 4025 50  0001 C CNN
F 4 "C117815" H 2475 4025 50  0001 C CNN "LCSC Part #"
	1    2475 4025
	1    0    0    -1  
$EndComp
$Comp
L Audio:TLV320AIC23BPW U2
U 1 1 608C10F7
P 8350 2300
F 0 "U2" H 8350 1211 50  0000 C CNN
F 1 "TLV320AIC23BPW" H 8350 1120 50  0000 C CNN
F 2 "Package_SO:TSSOP-28_4.4x9.7mm_P0.65mm" H 8350 2300 50  0001 C CIN
F 3 "http://www.ti.com/lit/ds/symlink/tlv320aic23b.pdf" H 8350 2300 50  0001 C CNN
F 4 "C9915" H 8350 2300 50  0001 C CNN "LCSC Part #"
	1    8350 2300
	1    0    0    -1  
$EndComp
$Comp
L Regulator_Linear:LM1117-3.3 U3
U 1 1 60908EF3
P 6400 4375
F 0 "U3" H 6400 4617 50  0000 C CNN
F 1 " LM1117F-3.3" H 6400 4526 50  0000 C CNN
F 2 "Package_TO_SOT_SMD:SOT-89-3" H 6400 4375 50  0001 C CNN
F 3 "http://www.ti.com/lit/ds/symlink/lm1117.pdf" H 6400 4375 50  0001 C CNN
F 4 "C126019" H 6400 4375 50  0001 C CNN "LCSC Part #"
	1    6400 4375
	1    0    0    -1  
$EndComp
$EndSCHEMATC
