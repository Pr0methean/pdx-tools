import React from "react";
import { Drawer } from "antd";
import { useUploadProgress, useUploadResponse } from "./uploadContext";
import { UploadDrawerContent } from "./UploadDrawerContent";
import { HelpTooltip } from "@/components/HelpTooltip";
import { ProgressBar } from "@/components/ProgressBar";
import { SaveMode } from "../../components/save-mode";
import { SuccessAlert } from "../../components/SuccessAlert";
import { closeDrawerPropagation } from "../../components/SideBarContainer";
import { useEu4Meta, useSaveFilename } from "../../store";

interface UploadDrawerProps {
  open: boolean;
  closeDrawer: () => void;
}

export const UploadDrawerTitle = () => {
  const meta = useEu4Meta();
  const filename = useSaveFilename();
  const progress = useUploadProgress();
  const uploadResponse = useUploadResponse();
  return (
    <div className="flex items-center gap-2">
      <SaveMode mode={meta.mode} />
      <span>{`Upload ${filename}`}</span>
      <HelpTooltip help="Upload the save to PDX Tools servers so you can share a link with the world" />
      <span className="grow">
        {progress !== undefined && <ProgressBar height={30} value={progress} />}
        {uploadResponse && <SuccessAlert newSaveId={uploadResponse.save_id} />}
      </span>
    </div>
  );
};

export const UploadDrawer = ({ open, closeDrawer }: UploadDrawerProps) => {
  return (
    <Drawer
      title={<UploadDrawerTitle />}
      placement="right"
      closable={true}
      mask={false}
      maskClosable={false}
      onClose={closeDrawerPropagation(closeDrawer, open)}
      open={open}
      width="min(800px, 100%)"
    >
      <UploadDrawerContent />
    </Drawer>
  );
};
