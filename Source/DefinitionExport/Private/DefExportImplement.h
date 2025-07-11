#pragma once

class DefExportImplement
{
private:
	FString Output;
	// void WriteProperty(FProperty* const Property);
	// void WriteStruct(UScriptStruct* const Struct);
public:
	DefExportImplement();
	void FetchDefinitions();
	bool WriteToFile();
};
