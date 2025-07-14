#pragma once

class Implement
{
private:
	FString Output;
	// void WriteProperty(FProperty* const Property);
	// void WriteStruct(UScriptStruct* const Struct);
public:
	Implement();
	void FetchDefinitions();
	bool WriteToFile(FString const &Path);
};
