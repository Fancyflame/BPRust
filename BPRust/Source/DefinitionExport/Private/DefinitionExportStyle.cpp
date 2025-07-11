// Copyright Epic Games, Inc. All Rights Reserved.

#include "DefinitionExportStyle.h"
#include "DefinitionExport.h"
#include "Framework/Application/SlateApplication.h"
#include "Styling/SlateStyleRegistry.h"
#include "Slate/SlateGameResources.h"
#include "Interfaces/IPluginManager.h"
#include "Styling/SlateStyleMacros.h"

#define RootToContentDir Style->RootToContentDir

TSharedPtr<FSlateStyleSet> FDefinitionExportStyle::StyleInstance = nullptr;

void FDefinitionExportStyle::Initialize()
{
	if (!StyleInstance.IsValid())
	{
		StyleInstance = Create();
		FSlateStyleRegistry::RegisterSlateStyle(*StyleInstance);
	}
}

void FDefinitionExportStyle::Shutdown()
{
	FSlateStyleRegistry::UnRegisterSlateStyle(*StyleInstance);
	ensure(StyleInstance.IsUnique());
	StyleInstance.Reset();
}

FName FDefinitionExportStyle::GetStyleSetName()
{
	static FName StyleSetName(TEXT("DefinitionExportStyle"));
	return StyleSetName;
}


const FVector2D Icon16x16(16.0f, 16.0f);
const FVector2D Icon20x20(20.0f, 20.0f);

TSharedRef< FSlateStyleSet > FDefinitionExportStyle::Create()
{
	TSharedRef< FSlateStyleSet > Style = MakeShareable(new FSlateStyleSet("DefinitionExportStyle"));
	Style->SetContentRoot(IPluginManager::Get().FindPlugin("BPRust")->GetBaseDir() / TEXT("Resources"));

	Style->Set("BPRust.DefExport.PluginAction", new IMAGE_BRUSH_SVG(TEXT("PlaceholderButtonIcon"), Icon20x20));
	return Style;
}

void FDefinitionExportStyle::ReloadTextures()
{
	if (FSlateApplication::IsInitialized())
	{
		FSlateApplication::Get().GetRenderer()->ReloadTextureResources();
	}
}

const ISlateStyle& FDefinitionExportStyle::Get()
{
	return *StyleInstance;
}
