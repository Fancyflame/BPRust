// Copyright Epic Games, Inc. All Rights Reserved.

#include "DefinitionExport.h"

#include "DefExportImplement.h"
#include "DefinitionExportStyle.h"
#include "DefinitionExportCommands.h"
#include "Misc/MessageDialog.h"
#include "ToolMenus.h"

static const FName DefinitionExportTabName("DefinitionExport");

#define LOCTEXT_NAMESPACE "FDefinitionExportModule"

void FDefinitionExportModule::StartupModule()
{
	// This code will execute after your module is loaded into memory; the exact timing is specified in the .uplugin file per-module
	
	FDefinitionExportStyle::Initialize();
	FDefinitionExportStyle::ReloadTextures();

	FDefinitionExportCommands::Register();
	
	PluginCommands = MakeShareable(new FUICommandList);

	PluginCommands->MapAction(
		FDefinitionExportCommands::Get().PluginAction,
		FExecuteAction::CreateRaw(this, &FDefinitionExportModule::PluginButtonClicked),
		FCanExecuteAction());

	UToolMenus::RegisterStartupCallback(FSimpleMulticastDelegate::FDelegate::CreateRaw(this, &FDefinitionExportModule::RegisterMenus));
}

void FDefinitionExportModule::ShutdownModule()
{
	// This function may be called during shutdown to clean up your module.  For modules that support dynamic reloading,
	// we call this function before unloading the module.

	UToolMenus::UnRegisterStartupCallback(this);

	UToolMenus::UnregisterOwner(this);

	FDefinitionExportStyle::Shutdown();

	FDefinitionExportCommands::Unregister();
}

void FDefinitionExportModule::PluginButtonClicked()
{
	// Put your "OnButtonClicked" stuff here
	DefExportImplement Exporter;
	Exporter.FetchDefinitions();
	const TCHAR* WriteResult = Exporter.WriteToFile() ? TEXT("succeed") : TEXT("failed");
	
	FText DialogText = FText::Format(
							LOCTEXT("PluginButtonDialogText", "Definition export {0}"),
							FText::FromString(WriteResult)
					   );
	FMessageDialog::Open(EAppMsgType::Ok, DialogText);
}

void FDefinitionExportModule::RegisterMenus()
{
	// Owner will be used for cleanup in call to UToolMenus::UnregisterOwner
	FToolMenuOwnerScoped OwnerScoped(this);

	{
		UToolMenu* Menu = UToolMenus::Get()->ExtendMenu("LevelEditor.MainMenu.Window");
		{
			FToolMenuSection& Section = Menu->FindOrAddSection("WindowLayout");
			Section.AddMenuEntryWithCommandList(FDefinitionExportCommands::Get().PluginAction, PluginCommands);
		}
	}

	{
		UToolMenu* ToolbarMenu = UToolMenus::Get()->ExtendMenu("LevelEditor.LevelEditorToolBar.PlayToolBar");
		{
			FToolMenuSection& Section = ToolbarMenu->FindOrAddSection("PluginTools");
			{
				FToolMenuEntry& Entry = Section.AddEntry(FToolMenuEntry::InitToolBarButton(FDefinitionExportCommands::Get().PluginAction));
				Entry.SetCommandList(PluginCommands);
			}
		}
	}
}

#undef LOCTEXT_NAMESPACE
	
IMPLEMENT_MODULE(FDefinitionExportModule, DefinitionExport)