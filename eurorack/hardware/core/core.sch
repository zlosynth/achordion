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
$Comp
L Connector_Generic:Conn_01x10 J1
U 1 1 60AE94CD
P 8425 4300
F 0 "J1" H 8505 4292 50  0000 L CNN
F 1 "ConnNW" H 8505 4201 50  0000 L CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x10_P2.54mm_Vertical" H 8425 4300 50  0001 C CNN
F 3 "~" H 8425 4300 50  0001 C CNN
	1    8425 4300
	1    0    0    -1  
$EndComp
$Comp
L Connector_Generic:Conn_01x10 J2
U 1 1 60AEC1D4
P 8425 5650
F 0 "J2" H 8505 5642 50  0000 L CNN
F 1 "ConnSW" H 8505 5551 50  0000 L CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x10_P2.54mm_Vertical" H 8425 5650 50  0001 C CNN
F 3 "~" H 8425 5650 50  0001 C CNN
	1    8425 5650
	1    0    0    -1  
$EndComp
$Comp
L Connector_Generic:Conn_01x10 J3
U 1 1 60AED02D
P 9900 4300
F 0 "J3" H 9980 4292 50  0000 L CNN
F 1 "ConnNE" H 9980 4201 50  0000 L CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x10_P2.54mm_Vertical" H 9900 4300 50  0001 C CNN
F 3 "~" H 9900 4300 50  0001 C CNN
	1    9900 4300
	1    0    0    -1  
$EndComp
$Comp
L Connector_Generic:Conn_01x10 J4
U 1 1 60AED5FD
P 9900 5650
F 0 "J4" H 9980 5642 50  0000 L CNN
F 1 "ConnSE" H 9980 5551 50  0000 L CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x10_P2.54mm_Vertical" H 9900 5650 50  0001 C CNN
F 3 "~" H 9900 5650 50  0001 C CNN
	1    9900 5650
	1    0    0    -1  
$EndComp
$Comp
L Connector_Generic:Conn_02x05_Odd_Even J5
U 1 1 60AF91AD
P 5500 1400
F 0 "J5" H 5550 1817 50  0000 C CNN
F 1 "Power" H 5550 1726 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_2x05_P2.54mm_Vertical" H 5500 1400 50  0001 C CNN
F 3 "~" H 5500 1400 50  0001 C CNN
	1    5500 1400
	1    0    0    -1  
$EndComp
$EndSCHEMATC
