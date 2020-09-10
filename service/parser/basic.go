package parser

func ParseUint8(data []byte) uint8 {
	result := uint8(data[0]);
	return result;
} 

func ParseUint16(data []byte) uint16 {
	result := (uint16(data[1]) << 8) + uint16(data[0]);
	return result;
}

func ParseUint32(data []byte) uint32 {
	result := (uint32(data[2]) << 16) + (uint32(data[1]) << 8) + uint32(data[0]);
	return result;
}