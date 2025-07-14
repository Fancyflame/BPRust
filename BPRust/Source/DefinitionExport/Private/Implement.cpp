#include "Implement.h"

#include "Interfaces/IPluginManager.h"

Implement::Implement()
{
}

void AppendComma(FString &Output, bool& ShouldAppendTrailingComma)
{
	if (ShouldAppendTrailingComma)
	{
		Output += ", ";
	}
	else
	{
		ShouldAppendTrailingComma = true;
	}
}

// -------- START Handle Property -------------------------------

const TCHAR* RecognizeParamType(EPropertyFlags Flags)
{
	return (Flags & CPF_OutParm) ? TEXT("out") : TEXT("in");
}

#define WRITE_PRIM_PROP_TYPE(__PropertyType, __RelativeType) \
	else if (Property->IsA<__PropertyType>()) { \
		PropertyType = TEXT("Primitive"); \
		RelativeType = TEXT(__RelativeType); \
	} \

bool WritePropertyType(FProperty* Property, const TCHAR* & PropertyType, FString& RelativeType)
{
	if (auto* ObjectProperty = CastField<FObjectProperty>(Property))
	{
		PropertyType = TEXT("Object");
		RelativeType = ObjectProperty->PropertyClass->GetAuthoredName();
	}
	else if (auto* StructProperty = CastField<FStructProperty>(Property))
	{
		PropertyType = TEXT("Struct");
		RelativeType = StructProperty->Struct->GetAuthoredName();
	}
	else if (auto* EnumProperty = CastField<FEnumProperty>(Property))
	{
		// if this needs to modify , don't forget to modify byte property part
		
		PropertyType = TEXT("Enum");
		// `GetAuthoredName()` always returns empty, use `GetName` instead.
		RelativeType = EnumProperty->GetEnum()->GetName();
	}
	else if (auto* Byte = CastField<FByteProperty>(Property))
	{
		if (Byte->IsEnum())
		{
			UEnum* Enum = Byte->Enum.Get();
			PropertyType = TEXT("Enum");
			RelativeType = Enum->GetName();
		} else
		{
			PropertyType = TEXT("Primitive");
			RelativeType = TEXT("Byte");
		}
	}
	WRITE_PRIM_PROP_TYPE(FNameProperty, "Name")
	WRITE_PRIM_PROP_TYPE(FStrProperty, "Str")
	WRITE_PRIM_PROP_TYPE(FTextProperty, "Text")
	WRITE_PRIM_PROP_TYPE(FBoolProperty, "Bool")
	WRITE_PRIM_PROP_TYPE(FIntProperty, "Int")
	WRITE_PRIM_PROP_TYPE(FInt64Property, "Int64")
	WRITE_PRIM_PROP_TYPE(FFloatProperty, "Float")
	WRITE_PRIM_PROP_TYPE(FDoubleProperty, "Double")
	else
	{
		// other types are not supported yet
		return false;
	}
	
	return true;
}

bool WriteProperty(FProperty* Property, FString& OutStr)
{
	const TCHAR* PropertyType;
	FString RelativeType;
	if (!WritePropertyType(Property, PropertyType, RelativeType))
	{
		return false;
	}
	
	FString Name = Property->GetAuthoredName(); //Property->GetName();
	// const TCHAR* ParamType = RecognizeParamType(Property->PropertyFlags);

	const TCHAR* Format = TEXT(
		R"JSON({
"name": "{0}",
"property": "{1}",
"type_info": "{2}",
"flags": {3}
})JSON"
	);
	OutStr = FString::Format(Format, {
		                         Name,
		                         PropertyType,
		                         RelativeType,
		                         Property->PropertyFlags
	                         }
	);
	return true;
}

template<class Iterator>
bool WritePropertiesArray(Iterator PropIt, FString &OutputJson, bool IgnoreUnsupported = false)
{
	OutputJson = TEXT("[");
	bool ShouldAppendComma = false;
	for (;PropIt;++PropIt)
	{
		FString MemberJson;
		if (!WriteProperty(*PropIt, MemberJson))
		{
			if (IgnoreUnsupported)
			{
				continue;
			}
			return false;
		}

		AppendComma(OutputJson, ShouldAppendComma);
		OutputJson += MemberJson;
	}
	OutputJson+="]";
	return true;
}

// -------- END Handle Property -------------------------------

FString WriteFunctions(UClass* Class)
{
	FString OutJson(TEXT("["));
	bool FunctionListAppendComma = false;
	
    for (TFieldIterator<UFunction> FuncIt(Class, EFieldIteratorFlags::ExcludeSuper); FuncIt; ++FuncIt)
    {
        UFunction* Func = *FuncIt;
    	FString Params;
    	if (!WritePropertiesArray(TFieldIterator<FProperty>(Func), Params))
    	{
    		continue;
    	}
        FString Name = Func->GetAuthoredName();

    	AppendComma(OutJson,FunctionListAppendComma);
    	const TCHAR* Format = TEXT(
		R"({
"name": "{0}",
"params": {1}
})"
		);
    	OutJson += FString::Format(Format, {Name, Params});
    }

	OutJson += TEXT("]");
	return OutJson;
}

FString WriteClass(UClass* const Class)
{
	FString ClassName = Class->GetAuthoredName();
	FString SuperClassName;
	UClass *SuperClass = Class->GetSuperClass(); 
	if (IsValid(SuperClass))
	{
		SuperClassName = SuperClass->GetAuthoredName();
	}
	
	FString Functions = WriteFunctions(Class);
	
	FString Properties;
	WritePropertiesArray(TFieldIterator<FProperty>(Class),Properties,true);

	const TCHAR* Format = TEXT(
		R"({
"name": "{0}",
"super": "{1}",
"properties": {2},
"functions": {3}
})"
	);
	return FString::Format(Format, {ClassName, SuperClassName, Properties, Functions});
}

bool WriteStruct(UScriptStruct* const Struct, FString &OutStr)
{
	// UClass* Type = Struct->StaticClass();
	FString StructName = Struct->GetAuthoredName();

	FString MembersStr;
	if (!WritePropertiesArray(TFieldIterator<FProperty>(Struct), MembersStr))
	{
		return false;
	}

	const TCHAR* Format = TEXT(
		R"JSON({
"name": "{0}",
"members": {1}
})JSON");
	OutStr = FString::Format(Format, {StructName, MembersStr});
	return true;
}

FString WriteEnum(UEnum* const Enum)
{
	FString Name = Enum->GetName();

	FString VariantsStr;
	bool MemberAppendComma = false;
	for (int32 i = 0; i < Enum->NumEnums(); ++i)
	{
		AppendComma(VariantsStr, MemberAppendComma);
		FString MemberName = Enum->GetAuthoredNameStringByIndex(i);
		int64 Value = Enum->GetValueByIndex(i);
		VariantsStr += FString::Format(TEXT("\n\"{0}\": {1}"), {MemberName, Value});
	}

	const TCHAR* Format = TEXT(
		R"({
"name": "{0}",
"variants": {{1}}
})"
	);
	return FString::Format(Format, {Name,VariantsStr});
}

// -------- START Basic -------------------------------

template<typename T>
void WriteSingleBasicType(FString &Out, const TCHAR* Name, bool &ShouldAppendComma)
{
	AppendComma(Out, ShouldAppendComma);
	Out.Appendf(TEXT(
R"("%s": {
"size": %d,
"align": %d
})"
	), Name, sizeof(T), alignof(T));
}

#define WRITE_SINGLE_BASIC_TYPE(__TypeDef, __TypeName) \
	WriteSingleBasicType<__TypeDef>(Out, TEXT(__TypeName), ShouldAppendComma)
void WriteBasicTypes(FString &Out)
{
	bool ShouldAppendComma = false;
	WRITE_SINGLE_BASIC_TYPE(FName, "FName");
	WRITE_SINGLE_BASIC_TYPE(FString, "FString");
	WRITE_SINGLE_BASIC_TYPE(FText, "FText");
	WRITE_SINGLE_BASIC_TYPE(FScriptArray, "FScriptArray");
	WRITE_SINGLE_BASIC_TYPE(FScriptSet, "FScriptSet");
	WRITE_SINGLE_BASIC_TYPE(FScriptMap, "FScriptMap");
	WRITE_SINGLE_BASIC_TYPE(FSoftObjectPtr, "FSoftObjectPtr");
}

// -------- END Basic ----------------------------------

void Implement::FetchDefinitions()
{
	Output = FString(TEXT("{"));
	
	// Class
	Output += TEXT("\"classes\": [");
	bool ClassComma = false;
	for (TObjectIterator<UClass> It; It; ++It)
	{
		AppendComma(Output,ClassComma);
		Output += WriteClass(*It);
	}
	Output += TEXT("],");

	// Struct
	Output += TEXT("\"structs\": [");
	bool StructComma = false;
	for (TObjectIterator<UScriptStruct> It; It; ++It)
	{
		FString Def;
		if (WriteStruct(*It, Def))
		{
			AppendComma(Output,StructComma);
			Output += Def;
		}
	}
	Output += TEXT("],");

	// Enum
	Output += TEXT("\"enums\": [");
	bool EnumComma = false;
	for (TObjectIterator<UEnum> It; It; ++It)
	{
		AppendComma(Output, EnumComma);
		Output += WriteEnum(*It);
	}
	Output+=TEXT("],");

	// Basic Struct
	Output += TEXT("\"basic_types\": {");
	WriteBasicTypes(Output);
	Output += TEXT("}");

	Output += TEXT("}");
	
}

bool Implement::WriteToFile(FString const &FilePath)
{
	// 将 FString 写入文件
	bool bSuccess = FFileHelper::SaveStringToFile(
		Output,
		*FilePath,
		FFileHelper::EEncodingOptions::AutoDetect,
		&IFileManager::Get(),
		EFileWrite::FILEWRITE_None
	);

	return bSuccess;
}
