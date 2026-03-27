// Function: FUN_ram_00000148
/* WARNING: Type propagation algorithm not settling */

ulonglong FUN_ram_00000148(ulonglong *param_1,ulonglong param_2,ulonglong param_3,ulonglong param_4,
                          ulonglong param_5)

{
  bool bVar1;
  bool bVar2;
  ulonglong uVar3;
  undefined8 *puVar4;
  longlong *plVar5;
  undefined8 uVar6;
  ulonglong uVar7;
  byte bVar8;
  undefined1 uVar9;
  undefined1 uVar10;
  longlong *plVar11;
  undefined8 *puVar12;
  char *pcVar13;
  byte *pbVar14;
  ulonglong uVar15;
  undefined1 uVar16;
  
  if ((longlong)param_5 < 0) {
    plVar5 = (longlong *)0x33680;
    FUN_ram_0002fbd8("assertion failed: min <= max/home/runner/work/platform-tools/platform-tools/out/rust/library/core/src/cmp.rsinternal error: entered unreachable code"
                     ,0x1c,&DAT_ram_000340b8);
    uVar3 = 0x1a;
    uVar15 = plVar5[9];
    uVar7 = plVar5[10];
    if (uVar15 < uVar7) {
      puVar12 = (undefined8 *)(plVar5[7] + uVar15 * 0x38);
      puVar4 = (undefined8 *)(*plVar5 + (uVar15 + plVar5[4]) * 8);
      pcVar13 = (char *)((uVar15 + plVar5[4]) * 0x10 + plVar5[2] + 8);
      do {
        pbVar14 = *(byte **)*puVar4;
        plVar11 = *(longlong **)(pcVar13 + -8);
        if ((((*(longlong *)(pbVar14 + 8) != *plVar11) ||
             (*(longlong *)(pbVar14 + 0x10) != plVar11[1])) ||
            (*(longlong *)(pbVar14 + 0x18) != plVar11[2])) ||
           (bVar1 = false, *(longlong *)(pbVar14 + 0x20) != plVar11[3])) {
          bVar1 = true;
        }
        uVar15 = uVar15 + 1;
        if (bVar1) {
          uVar3 = 1;
          goto LAB_ram_00000590;
        }
        bVar8 = 0;
        if (*pcVar13 == '\0') {
          bVar8 = 0x77;
        }
        if ((bVar8 | *pbVar14) != 0xff) {
          uVar3 = 0xb;
          goto LAB_ram_00000590;
        }
        uVar16 = 1;
        if (pbVar14[1] == 0) {
          uVar16 = 0;
          if (pbVar14[2] == 0) goto LAB_ram_00000530;
LAB_ram_000004f0:
          uVar10 = 1;
          uVar9 = 1;
          bVar8 = pbVar14[3];
        }
        else {
          if (pbVar14[2] != 0) goto LAB_ram_000004f0;
LAB_ram_00000530:
          uVar10 = 0;
          uVar9 = 0;
          bVar8 = pbVar14[3];
        }
        if (bVar8 == 0) {
          uVar9 = uVar10;
        }
        uVar6 = *(undefined8 *)(pbVar14 + 0x50);
        puVar12[4] = pbVar14 + 0x28;
        puVar12[3] = pbVar14 + 0x58;
        puVar12[2] = uVar6;
        puVar12[1] = pbVar14 + 0x48;
        *puVar12 = pbVar14 + 8;
        *(bool *)((longlong)puVar12 + 0x32) = bVar8 != 0;
        *(undefined1 *)((longlong)puVar12 + 0x31) = uVar9;
        *(undefined1 *)(puVar12 + 6) = uVar16;
        pcVar13 = pcVar13 + 0x10;
        puVar4 = puVar4 + 1;
        puVar12[5] = 0;
        puVar12 = puVar12 + 7;
      } while (uVar15 < uVar7);
      uVar3 = 0x1a;
      uVar15 = uVar7;
LAB_ram_00000590:
      plVar5[9] = uVar15;
    }
    return uVar3;
  }
  bVar1 = true;
  if (param_2 < param_4) {
    if (param_3 < param_5) goto LAB_ram_00000178;
LAB_ram_000001e0:
    bVar2 = false;
    if (param_3 != param_5) goto LAB_ram_000001f0;
LAB_ram_00000180:
    if (bVar1) goto joined_r0x00000210;
  }
  else {
    bVar1 = false;
    if (param_5 <= param_3) goto LAB_ram_000001e0;
LAB_ram_00000178:
    bVar2 = true;
    if (param_3 == param_5) goto LAB_ram_00000180;
LAB_ram_000001f0:
    bVar1 = bVar2;
    if (bVar1) goto joined_r0x00000210;
  }
  param_2 = param_4;
joined_r0x00000210:
  if (bVar1) {
    param_5 = param_3;
  }
  if ((longlong)param_3 < 0) {
    param_2 = 0;
    param_5 = 0;
  }
  param_1[1] = param_5;
  *param_1 = param_2;
  return param_5;
}

// Function: FUN_ram_00000278
undefined8 FUN_ram_00000278(longlong *param_1)

{
  bool bVar1;
  undefined8 uVar2;
  undefined8 *puVar3;
  ulonglong uVar4;
  byte bVar5;
  undefined1 uVar6;
  undefined1 uVar7;
  longlong *plVar8;
  undefined8 *puVar9;
  char *pcVar10;
  byte *pbVar11;
  ulonglong uVar12;
  undefined1 uVar13;
  
  uVar2 = 0x1a;
  uVar12 = param_1[9];
  uVar4 = param_1[10];
  if (uVar12 < uVar4) {
    puVar9 = (undefined8 *)(param_1[7] + uVar12 * 0x38);
    puVar3 = (undefined8 *)(*param_1 + (uVar12 + param_1[4]) * 8);
    pcVar10 = (char *)((uVar12 + param_1[4]) * 0x10 + param_1[2] + 8);
    do {
      pbVar11 = *(byte **)*puVar3;
      plVar8 = *(longlong **)(pcVar10 + -8);
      if ((((*(longlong *)(pbVar11 + 8) != *plVar8) || (*(longlong *)(pbVar11 + 0x10) != plVar8[1]))
          || (*(longlong *)(pbVar11 + 0x18) != plVar8[2])) ||
         (bVar1 = false, *(longlong *)(pbVar11 + 0x20) != plVar8[3])) {
        bVar1 = true;
      }
      uVar12 = uVar12 + 1;
      if (bVar1) {
        uVar2 = 1;
        goto LAB_ram_00000590;
      }
      bVar5 = 0;
      if (*pcVar10 == '\0') {
        bVar5 = 0x77;
      }
      if ((bVar5 | *pbVar11) != 0xff) {
        uVar2 = 0xb;
        goto LAB_ram_00000590;
      }
      uVar13 = 1;
      if (pbVar11[1] == 0) {
        uVar13 = 0;
        if (pbVar11[2] == 0) goto LAB_ram_00000530;
LAB_ram_000004f0:
        uVar7 = 1;
        uVar6 = 1;
        bVar5 = pbVar11[3];
      }
      else {
        if (pbVar11[2] != 0) goto LAB_ram_000004f0;
LAB_ram_00000530:
        uVar7 = 0;
        uVar6 = 0;
        bVar5 = pbVar11[3];
      }
      if (bVar5 == 0) {
        uVar6 = uVar7;
      }
      uVar2 = *(undefined8 *)(pbVar11 + 0x50);
      puVar9[4] = pbVar11 + 0x28;
      puVar9[3] = pbVar11 + 0x58;
      puVar9[2] = uVar2;
      puVar9[1] = pbVar11 + 0x48;
      *puVar9 = pbVar11 + 8;
      *(bool *)((longlong)puVar9 + 0x32) = bVar5 != 0;
      *(undefined1 *)((longlong)puVar9 + 0x31) = uVar6;
      *(undefined1 *)(puVar9 + 6) = uVar13;
      pcVar10 = pcVar10 + 0x10;
      puVar3 = puVar3 + 1;
      puVar9[5] = 0;
      puVar9 = puVar9 + 7;
    } while (uVar12 < uVar4);
    uVar2 = 0x1a;
    uVar12 = uVar4;
LAB_ram_00000590:
    param_1[9] = uVar12;
  }
  return uVar2;
}

// Function: FUN_ram_000005a8
void FUN_ram_000005a8(ulonglong *param_1,longlong param_2,ulonglong param_3,longlong param_4,
                     ulonglong param_5)

{
  bool bVar1;
  bool bVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  longlong lVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong local_40;
  longlong local_38;
  ulonglong local_30;
  longlong local_28;
  longlong local_20;
  longlong local_18;
  ulonglong local_10;
  longlong local_8;
  
  FUN_ram_00031e70(&local_40,param_3,(longlong)param_3 >> 0x3f);
  FUN_ram_00031e70(&local_30,param_4,0,param_2,0);
  FUN_ram_00031e70(&local_10,param_5,(longlong)param_5 >> 0x3f,param_2,param_2 >> 0x3f);
  FUN_ram_00031e70(&local_20,param_5,(longlong)param_5 >> 0x3f,param_3,(longlong)param_3 >> 0x3f);
  uVar3 = local_10 + local_40 + local_28;
  lVar5 = local_8 + (param_2 >> 0x3f & param_5) + (ulonglong)(uVar3 < local_10);
  uVar6 = local_38 + (param_4 >> 0x3f & param_3) + (ulonglong)(local_40 + local_28 < local_40);
  uVar7 = uVar6 + local_20;
  uVar4 = uVar7 + lVar5;
  lVar5 = ((longlong)uVar6 >> 0x3f) + local_18 + (ulonglong)(uVar7 < uVar6) + (lVar5 >> 0x3f) +
          (ulonglong)(uVar4 < uVar7);
  uVar6 = (longlong)(uVar4 * 0x10000) >> 0x3f;
  bVar1 = (uVar4 >> 0x30 | lVar5 * 0x10000) == uVar6;
  bVar2 = lVar5 >> 0x30 == uVar6;
  if (bVar1 && bVar2) {
    param_1[1] = uVar3 * 0x10000 | local_30 >> 0x30;
    param_1[2] = uVar4 * 0x10000 | uVar3 >> 0x30;
  }
  *param_1 = (ulonglong)(bVar1 && bVar2);
  return;
}

// Function: FUN_ram_00000908
void FUN_ram_00000908(void)

{
  longlong in_R4;
  longlong in_R5;
  undefined *local_30;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_18;
  undefined8 local_10;
  
  if (in_R4 != 0 || in_R5 != 0) {
    FUN_ram_0002df08();
    return;
  }
  local_30 = &DAT_ram_000340d0;
  local_10 = 0;
  local_28 = 1;
  local_18 = 0;
  local_20 = 8;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&local_30,&DAT_ram_000340e0);
}

// Function: FUN_ram_000009a8
void FUN_ram_000009a8(longlong *param_1,undefined *param_2,ulonglong param_3)

{
  bool bVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  undefined *puVar4;
  longlong lVar5;
  bool bVar6;
  undefined *puVar7;
  undefined *puVar8;
  undefined *local_40;
  undefined8 local_38;
  undefined **local_30;
  longlong local_28;
  ulonglong local_20;
  undefined *local_10;
  ulonglong local_8;
  
  if ((longlong)param_3 < 0) {
    lVar5 = 0x2c;
    FUN_ram_0002fbd8(&DAT_ram_0003378f,0x2c,&DAT_ram_000340f8);
    local_40 = &DAT_ram_00034120;
    local_30 = &local_10;
    local_8 = 0x120;
    local_10 = &DAT_ram_00034110;
    local_20 = 0;
    local_38 = 1;
    local_28 = 1;
    FUN_ram_0002fdf0(*(undefined8 *)(lVar5 + 0x20),*(undefined8 *)(lVar5 + 0x28),&local_40);
    return;
  }
  local_30 = (undefined **)param_1;
  if (param_2 == (undefined *)0x0 && param_3 == 0) {
    lVar5 = 0;
    uVar3 = 0;
  }
  else {
    if (param_3 == 0) {
      uVar3 = (ulonglong)param_2 | (ulonglong)param_2 >> 1;
      uVar3 = uVar3 | uVar3 >> 2;
      uVar3 = uVar3 | uVar3 >> 4;
      uVar3 = uVar3 | uVar3 >> 8;
      uVar3 = uVar3 | uVar3 >> 0x10;
      uVar3 = (uVar3 | uVar3 >> 0x20) ^ 0xffffffffffffffff;
      uVar3 = uVar3 - (uVar3 >> 1 & 0x5555555555555555);
      uVar3 = (uVar3 & 0x3333333333333333) + (uVar3 >> 2 & 0x3333333333333333);
      uVar3 = ((uVar3 + (uVar3 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38) + 0x40;
    }
    else {
      uVar3 = param_3 | param_3 >> 1;
      uVar3 = uVar3 | uVar3 >> 2;
      uVar3 = uVar3 | uVar3 >> 4;
      uVar3 = uVar3 | uVar3 >> 8;
      uVar3 = uVar3 | uVar3 >> 0x10;
      uVar3 = (uVar3 | uVar3 >> 0x20) ^ 0xffffffffffffffff;
      uVar3 = uVar3 - (uVar3 >> 1 & 0x5555555555555555);
      uVar3 = (uVar3 & 0x3333333333333333) + (uVar3 >> 2 & 0x3333333333333333);
      uVar3 = (uVar3 + (uVar3 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38;
    }
    puVar8 = (undefined *)0x0;
    FUN_ram_00031e28(&local_10,1,0,(uVar3 ^ 0xffffffffffffffff) & 0x7e);
    uVar3 = 0;
    do {
      puVar4 = local_10 + (longlong)puVar8;
      local_20 = local_8 + uVar3 + (ulonglong)(puVar4 < local_10);
      bVar6 = true;
      if (param_3 < local_20) {
        if (param_2 < puVar4) goto LAB_ram_00000dc0;
LAB_ram_00000ef8:
        bVar1 = false;
        if (param_3 != local_20) goto LAB_ram_00000f08;
LAB_ram_00000dc8:
        if (bVar1) goto LAB_ram_00000dd8;
LAB_ram_00000f20:
        uVar2 = local_8;
        if (!bVar1) goto LAB_ram_00000f38;
LAB_ram_00000de8:
        local_20 = 0;
        if (bVar1) {
LAB_ram_00000e00:
          puVar4 = (undefined *)0x0;
        }
      }
      else {
        bVar6 = false;
        if (puVar4 <= param_2) goto LAB_ram_00000ef8;
LAB_ram_00000dc0:
        bVar1 = true;
        if (param_3 == local_20) goto LAB_ram_00000dc8;
LAB_ram_00000f08:
        bVar1 = bVar6;
        if (!bVar1) goto LAB_ram_00000f20;
LAB_ram_00000dd8:
        uVar2 = 0;
        if (bVar1) goto LAB_ram_00000de8;
LAB_ram_00000f38:
        if (bVar1) goto LAB_ram_00000e00;
      }
      puVar7 = (undefined *)0x0;
      if (!bVar1) {
        puVar7 = local_10;
      }
      puVar8 = puVar7 + ((ulonglong)puVar8 >> 1 | uVar3 << 0x3f);
      if (puVar8 < puVar7) {
        local_28 = 1;
        if (param_2 < puVar4) goto LAB_ram_00000e78;
LAB_ram_00000fa0:
        lVar5 = 0;
        if (local_10 < (undefined *)0x4) goto LAB_ram_00000fb8;
LAB_ram_00000e88:
        bVar6 = true;
      }
      else {
        local_28 = 0;
        if (puVar4 <= param_2) goto LAB_ram_00000fa0;
LAB_ram_00000e78:
        lVar5 = 1;
        if ((undefined *)0x3 < local_10) goto LAB_ram_00000e88;
LAB_ram_00000fb8:
        bVar6 = false;
      }
      if (local_8 != 0) {
        bVar6 = local_8 != 0;
      }
      param_3 = (param_3 - local_20) - lVar5;
      uVar3 = uVar2 + (uVar3 >> 1) + local_28;
      param_2 = param_2 + -(longlong)puVar4;
      local_10 = (undefined *)((ulonglong)local_10 >> 2 | local_8 << 0x3e);
      local_8 = local_8 >> 2;
    } while (bVar6);
    uVar3 = uVar3 * 0x1000000 | (ulonglong)puVar8 >> 0x28;
    lVar5 = (longlong)puVar8 * 0x1000000;
  }
  *local_30 = (undefined *)lVar5;
  ((longlong *)local_30)[1] = uVar3;
  return;
}

// Function: FUN_ram_000011b0
void FUN_ram_000011b0(undefined8 param_1,undefined8 param_2,undefined1 param_3)

{
  undefined8 local_68;
  undefined8 local_60;
  undefined1 local_51;
  undefined *local_50;
  undefined8 local_48;
  undefined8 **local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 *local_20;
  undefined8 local_18;
  undefined1 *local_10;
  undefined1 *local_8;
  
  local_50 = &DAT_ram_000341b0;
  local_40 = &local_20;
  local_8 = &LAB_ram_0002f968;
  local_10 = &local_51;
  local_18 = 0x120;
  local_20 = &local_68;
  local_30 = 0;
  local_48 = 2;
  local_38 = 2;
  local_68 = param_1;
  local_60 = param_2;
  local_51 = param_3;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&local_50,&DAT_ram_000341d0);
}

// Function: FUN_ram_00001298
void FUN_ram_00001298(longlong *param_1,ulonglong *param_2,undefined8 *param_3,ulonglong param_4)

{
  ulonglong uVar1;
  ulonglong uVar2;
  undefined1 uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined *local_80;
  longlong local_78;
  longlong local_70;
  longlong local_68;
  longlong local_60;
  longlong local_58;
  longlong local_50;
  longlong local_48;
  longlong local_40;
  longlong local_38;
  undefined8 local_30;
  undefined8 local_28;
  longlong local_20 [4];
  
  local_98 = param_3[1];
  local_a0 = *param_3;
  uVar4 = param_2[1];
  uVar5 = *param_2;
  uVar2 = param_4 & 0xffffffff;
  local_20[3] = 0;
  local_20[2] = 0;
  local_20[1] = 0;
  local_20[0] = 0;
  if (uVar2 < 0x100) {
    uVar6 = param_4 & 0x3f;
    uVar1 = uVar2 >> 6;
    local_20[uVar1] = uVar5 << uVar6;
    if (uVar2 < 0xc0) {
      local_20[uVar1 + 1] = uVar4 << uVar6;
      if (((param_4 & 0xffffffff) < 0x80) &&
         (local_20[uVar1 + 2] = 0, (param_4 & 0xffffffff) < 0x40)) {
        local_20[3] = 0;
      }
      if (uVar6 != 0) {
        local_20[uVar1 + 1] = local_20[uVar1 + 1] + (uVar5 >> (-uVar2 & 0x3f));
        if ((param_4 & 0xffffffff) < 0x80) {
          local_20[uVar1 + 2] = local_20[uVar1 + 2] + (uVar4 >> (-uVar2 & 0x3f));
        }
      }
    }
  }
  local_88 = 0;
  local_90 = 0;
  FUN_ram_0002a890(&local_80,local_20,&local_a0);
  local_38 = local_68;
  local_40 = local_70;
  local_30 = 0;
  local_28 = 0;
  if ((local_70 == 0) && (local_68 == 0)) {
    uVar3 = 0;
  }
  else {
    uVar3 = 1;
  }
  if (((((local_60 != 0) || (local_58 != 0)) || (local_50 != 0)) || (local_48 != 0)) &&
     ((local_80 = local_80 + 1, local_80 == (undefined *)0x0 &&
      (local_78 = local_78 + 1, local_78 == 0)))) {
    local_80 = &DAT_ram_00034648;
    local_60 = 0;
    local_78 = 1;
    local_68 = 0;
    local_70 = 8;
                    /* WARNING: Subroutine does not return */
    FUN_ram_0002fba8(&local_80,&DAT_ram_00034630);
  }
  *(undefined1 *)(param_1 + 2) = uVar3;
  param_1[1] = local_78;
  *param_1 = (longlong)local_80;
  return;
}

// Function: FUN_ram_00001708
void FUN_ram_00001708(undefined8 *param_1,ulonglong *param_2,longlong param_3,ulonglong param_4)

{
  ulonglong uVar1;
  ulonglong uVar2;
  undefined8 uVar3;
  bool bVar4;
  ulonglong uVar5;
  bool bVar6;
  ulonglong uVar7;
  longlong local_a0;
  ulonglong local_98;
  undefined8 local_90;
  undefined8 local_88;
  longlong local_60;
  ulonglong local_58;
  longlong local_50;
  longlong local_48;
  longlong local_20;
  ulonglong local_18;
  ulonglong local_10;
  undefined8 local_8;
  
  if (param_3 == 0 && param_4 == 0) {
    uVar3 = 0;
    goto LAB_ram_00001ae8;
  }
  if (param_3 == 0) {
    uVar5 = *param_2;
  }
  else {
    uVar5 = *param_2;
  }
  local_98 = param_4;
  local_a0 = param_3;
  if ((longlong)param_4 < 0) {
    local_a0 = -param_3;
    local_98 = -(param_4 + (param_3 != 0));
  }
  uVar7 = param_2[1];
  uVar1 = -uVar5;
  if (-1 < (longlong)uVar7) {
    uVar1 = uVar5;
  }
  local_20 = uVar1 << 0x30;
  uVar2 = uVar7;
  if ((longlong)uVar7 < 0) {
    uVar2 = -(uVar7 + (uVar5 != 0));
  }
  local_10 = uVar2 >> 0x10;
  local_18 = uVar2 << 0x30 | uVar1 >> 0x10;
  local_8 = 0;
  local_90 = 0;
  local_88 = 0;
  FUN_ram_0002a890(&local_60,&local_20,&local_a0);
  if ((local_50 == 0) && (local_48 == 0)) {
    bVar4 = false;
  }
  else {
    bVar4 = true;
  }
  uVar3 = 0;
  if (bVar4) goto LAB_ram_00001ae8;
  if ((longlong)(uVar7 ^ param_4) < 0) {
    bVar6 = true;
    bVar4 = true;
    if (local_60 == 0) {
      bVar4 = false;
      if (local_58 < 0x8000000000000001) goto LAB_ram_00001a58;
LAB_ram_00001a18:
      if (local_58 == 0x8000000000000000) goto LAB_ram_00001a20;
LAB_ram_00001a68:
      if (bVar6) goto LAB_ram_00001ae8;
    }
    else {
      if (0x8000000000000000 < local_58) goto LAB_ram_00001a18;
LAB_ram_00001a58:
      bVar6 = false;
      if (local_58 != 0x8000000000000000) goto LAB_ram_00001a68;
LAB_ram_00001a20:
      if (bVar4) goto LAB_ram_00001ae8;
    }
    local_60 = -local_60;
    uVar5 = local_58 ^ 0xffffffffffffffff;
    if (local_60 == 0) {
      uVar5 = -local_58;
    }
  }
  else {
    uVar5 = local_58;
    if ((longlong)local_58 < 0) goto LAB_ram_00001ae8;
  }
  param_1[1] = local_60;
  param_1[2] = uVar5;
  uVar3 = 1;
LAB_ram_00001ae8:
  *param_1 = uVar3;
  return;
}

// Function: FUN_ram_00001af8
/* WARNING: Type propagation algorithm not settling */

void FUN_ram_00001af8(undefined8 *param_1,longlong *param_2,longlong param_3,ulonglong param_4)

{
  longlong lVar1;
  bool bVar2;
  bool bVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  longlong local_38;
  ulonglong local_30;
  char local_28;
  longlong local_20;
  ulonglong local_18;
  longlong local_10;
  ulonglong local_8;
  
  if (param_3 == 0 && param_4 == 0) {
    uVar4 = 0;
    goto LAB_ram_00001de8;
  }
  uVar5 = param_2[1];
  lVar1 = *param_2;
  local_20 = -lVar1;
  if (-1 < (longlong)uVar5) {
    local_20 = lVar1;
  }
  uVar4 = 0;
  local_18 = uVar5;
  if ((longlong)uVar5 < 0) {
    local_18 = -(uVar5 + (lVar1 != 0));
  }
  local_10 = param_3;
  if ((longlong)param_4 < 0) {
    local_10 = -param_3;
  }
  local_8 = param_4;
  if ((longlong)param_4 < 0) {
    local_8 = -(param_4 + (param_3 != 0));
  }
  FUN_ram_00001298(&local_38,&local_20,&local_10,0x30);
  if (local_28 == '\x01') goto LAB_ram_00001de8;
  if ((longlong)(uVar5 ^ param_4) < 0) {
    bVar3 = true;
    bVar2 = true;
    if (local_38 == 0) {
      bVar2 = false;
      if (0x8000000000000000 < local_30) goto LAB_ram_00001d18;
LAB_ram_00001d58:
      bVar3 = false;
      if (local_30 == 0x8000000000000000) goto LAB_ram_00001d20;
LAB_ram_00001d68:
      if (bVar3) goto LAB_ram_00001de8;
    }
    else {
      if (local_30 < 0x8000000000000001) goto LAB_ram_00001d58;
LAB_ram_00001d18:
      if (local_30 != 0x8000000000000000) goto LAB_ram_00001d68;
LAB_ram_00001d20:
      if (bVar2) goto LAB_ram_00001de8;
    }
    local_38 = -local_38;
    uVar5 = local_30 ^ 0xffffffffffffffff;
    if (local_38 == 0) {
      uVar5 = -local_30;
    }
  }
  else {
    uVar5 = local_30;
    if ((longlong)local_30 < 0) goto LAB_ram_00001de8;
  }
  param_1[1] = local_38;
  param_1[2] = uVar5;
  uVar4 = 1;
LAB_ram_00001de8:
  *param_1 = uVar4;
  return;
}

// Function: FUN_ram_00001df8
void FUN_ram_00001df8(ulonglong *param_1,ulonglong param_2)

{
  ulonglong *puVar1;
  ulonglong uVar2;
  longlong lVar3;
  ulonglong uVar4;
  ulonglong local_8;
  
  uVar4 = 0;
  uVar2 = param_2 & 0xfffffffffffffff8;
  if (uVar2 != 0) {
    lVar3 = -uVar2;
    puVar1 = param_1;
    do {
      *puVar1 = uVar4 ^ *puVar1 ^ 0xc3ebbae2ff2fff3a;
      uVar4 = uVar4 + 0x1000100010001;
      puVar1 = puVar1 + 1;
      lVar3 = lVar3 + 8;
    } while (lVar3 != 0);
  }
  local_8 = 0;
  FUN_ram_00031b28(&local_8,(longlong)param_1 + uVar2,param_2 & 7);
  local_8 = uVar4 ^ local_8 ^ 0xc3ebbae2ff2fff3a;
  FUN_ram_00031b28((longlong)param_1 + uVar2,&local_8,param_2 & 7);
  return;
}

// Function: FUN_ram_00001f40
void FUN_ram_00001f40(ulonglong *param_1)

{
  *param_1 = *param_1 ^ 0xef4a578c67d5f08b;
  param_1[1] = param_1[1] ^ 0xef4b578d67d4f08a;
  param_1[2] = param_1[2] ^ 0xef48578e67d7f089;
  param_1[3] = param_1[3] ^ 0xef49578f67d6f088;
  param_1[4] = param_1[4] ^ 0xef4e578867d1f08f;
  param_1[5] = param_1[5] ^ 0xef4f578967d0f08e;
  param_1[6] = param_1[6] ^ 0xef4c578a67d3f08d;
  param_1[7] = param_1[7] ^ 0xef4d578b67d2f08c;
  param_1[8] = param_1[8] ^ 0xef42578467ddf083;
  param_1[9] = param_1[9] ^ 0xef43578567dcf082;
  param_1[10] = param_1[10] ^ 0xef40578667dff081;
  param_1[0xb] = param_1[0xb] ^ 0xef41578767def080;
  param_1[0xc] = param_1[0xc] ^ 0xef46578067d9f087;
  param_1[0xd] = param_1[0xd] ^ 0xef47578167d8f086;
  param_1[0xe] = param_1[0xe] ^ 0xef44578267dbf085;
  param_1[0xf] = param_1[0xf] ^ 0xef45578367daf084;
  param_1[0x10] = param_1[0x10] ^ 0xef5a579c67c5f09b;
  param_1[0x11] = param_1[0x11] ^ 0xef5b579d67c4f09a;
  param_1[0x12] = param_1[0x12] ^ 0xef58579e67c7f099;
  param_1[0x13] = param_1[0x13] ^ 0xef59579f67c6f098;
  param_1[0x14] = param_1[0x14] ^ 0xef5e579867c1f09f;
  param_1[0x15] = param_1[0x15] ^ 0xef5f579967c0f09e;
  param_1[0x16] = param_1[0x16] ^ 0xef5c579a67c3f09d;
  param_1[0x17] = param_1[0x17] ^ 0xef5d579b67c2f09c;
  param_1[0x18] = param_1[0x18] ^ 0xef52579467cdf093;
  param_1[0x19] = param_1[0x19] ^ 0xef53579567ccf092;
  param_1[0x1a] = param_1[0x1a] ^ 0xef50579667cff091;
  param_1[0x1b] = param_1[0x1b] ^ 0xef51579767cef090;
  param_1[0x1c] = param_1[0x1c] ^ 0xef56579067c9f097;
  param_1[0x1d] = param_1[0x1d] ^ 0xef57579167c8f096;
  param_1[0x1e] = param_1[0x1e] ^ 0xef54579267cbf095;
  param_1[0x1f] = param_1[0x1f] ^ 0xef55579367caf094;
  return;
}

// Function: FUN_ram_00002448
void FUN_ram_00002448(ulonglong *param_1)

{
  *param_1 = *param_1 ^ 0x69d190c683eda5d3;
  param_1[1] = param_1[1] ^ 0x962f6f387c135a2c;
  param_1[2] = param_1[2] ^ 0x962c6f3b7c105a2d;
  param_1[3] = param_1[3] ^ 0x962d6f3a7c115a2e;
  param_1[4] = param_1[4] ^ 0x962a6f3d7c165a2f;
  param_1[5] = param_1[5] ^ 0x962b6f3c7c175a28;
  param_1[6] = param_1[6] ^ 0x96286f3f7c145a29;
  param_1[7] = param_1[7] ^ 0x96296f3e7c155a2a;
  param_1[8] = param_1[8] ^ 0x96266f317c1a5a2b;
  param_1[9] = param_1[9] ^ 0x96276f307c1b5a24;
  param_1[10] = param_1[10] ^ 0x96246f337c185a25;
  param_1[0xb] = param_1[0xb] ^ 0x96256f327c195a26;
  param_1[0xc] = param_1[0xc] ^ 0x96226f357c1e5a27;
  param_1[0xd] = param_1[0xd] ^ 0x96236f347c1f5a20;
  param_1[0xe] = param_1[0xe] ^ 0x96206f377c1c5a21;
  param_1[0xf] = param_1[0xf] ^ 0x96216f367c1d5a22;
  param_1[0x10] = param_1[0x10] ^ 0x963e6f297c025a23;
  param_1[0x11] = param_1[0x11] ^ 0x963f6f287c035a3c;
  return;
}

// Function: FUN_ram_00002720
void FUN_ram_00002720(ulonglong *param_1)

{
  *param_1 = *param_1 ^ 0xdbf169454ad22fa;
  param_1[1] = param_1[1] ^ 0xf241e96aab522d05;
  param_1[2] = param_1[2] ^ 0xf242e969ab532d04;
  param_1[3] = param_1[3] ^ 0xf243e968ab502d07;
  param_1[4] = param_1[4] ^ 0xf244e96fab512d06;
  param_1[5] = param_1[5] ^ 0xf245e96eab562d01;
  param_1[6] = param_1[6] ^ 0xf246e96dab572d00;
  param_1[7] = param_1[7] ^ 0xf247e96cab542d03;
  param_1[8] = param_1[8] ^ 0xf248e963ab552d02;
  param_1[9] = param_1[9] ^ 0xf249e962ab5a2d0d;
  return;
}

// Function: FUN_ram_000028b8
void FUN_ram_000028b8(ulonglong *param_1)

{
  *param_1 = *param_1 ^ 0xb82c93d08854ebff;
  param_1[1] = param_1[1] ^ 0x47d26c2e77aa1400;
  param_1[2] = param_1[2] ^ 0x47d16c2d77a91401;
  param_1[3] = param_1[3] ^ 0x47d06c2c77a81402;
  param_1[4] = param_1[4] ^ 0x47d76c2b77af1403;
  param_1[5] = param_1[5] ^ 0x47d66c2a77ae1404;
  param_1[6] = param_1[6] ^ 0x47d56c2977ad1405;
  param_1[7] = param_1[7] ^ 0x47d46c2877ac1406;
  param_1[8] = param_1[8] ^ 0x47db6c2777a31407;
  param_1[9] = param_1[9] ^ 0x47da6c2677a21408;
  return;
}

// Function: FUN_ram_00002a50
void FUN_ram_00002a50(ulonglong *param_1)

{
  *param_1 = *param_1 ^ 0xbf03b62bffacf846;
  param_1[1] = param_1[1] ^ 0x40fd49d5005207b9;
  param_1[2] = param_1[2] ^ 0x40fe49d6005107b8;
  param_1[3] = param_1[3] ^ 0x40ff49d7005007bb;
  param_1[4] = param_1[4] ^ 0x40f849d0005707ba;
  param_1[5] = param_1[5] ^ 0x40f949d1005607bd;
  param_1[6] = param_1[6] ^ 0x40fa49d2005507bc;
  param_1[7] = param_1[7] ^ 0x40fb49d3005407bf;
  param_1[8] = param_1[8] ^ 0x40f449dc005b07be;
  param_1[9] = param_1[9] ^ 0x40f549dd005a07b1;
  param_1[10] = param_1[10] ^ 0x40f649de005907b0;
  param_1[0xb] = param_1[0xb] ^ 0x40f749df005807b3;
  param_1[0xc] = param_1[0xc] ^ 0x40f049d8005f07b2;
  param_1[0xd] = param_1[0xd] ^ 0x40f149d9005e07b5;
  param_1[0xe] = param_1[0xe] ^ 0x40f249da005d07b4;
  param_1[0xf] = param_1[0xf] ^ 0x40f349db005c07b7;
  param_1[0x10] = param_1[0x10] ^ 0x40ec49c4004307b6;
  param_1[0x11] = param_1[0x11] ^ 0x40ed49c5004207a9;
  param_1[0x12] = param_1[0x12] ^ 0x40ee49c6004107a8;
  param_1[0x13] = param_1[0x13] ^ 0x40ef49c7004007ab;
  param_1[0x14] = param_1[0x14] ^ 0x40e849c0004707aa;
  param_1[0x15] = param_1[0x15] ^ 0x40e949c1004607ad;
  param_1[0x16] = param_1[0x16] ^ 0x40ea49c2004507ac;
  param_1[0x17] = param_1[0x17] ^ 0x40eb49c3004407af;
  param_1[0x18] = param_1[0x18] ^ 0x40e449cc004b07ae;
  param_1[0x19] = param_1[0x19] ^ 0x40e549cd004a07a1;
  param_1[0x1a] = param_1[0x1a] ^ 0x40e649ce004907a0;
  param_1[0x1b] = param_1[0x1b] ^ 0x40e749cf004807a3;
  param_1[0x1c] = param_1[0x1c] ^ 0x40e049c8004f07a2;
  param_1[0x1d] = param_1[0x1d] ^ 0x40e149c9004e07a5;
  return;
}

// Function: entrypoint
ulonglong entrypoint(ulonglong *param_1)

{
  byte bVar1;
  bool bVar2;
  int iVar3;
  ulonglong in_R0;
  ulonglong *puVar4;
  ulonglong **ppuVar5;
  ulonglong uVar6;
  longlong lVar7;
  ulonglong uVar8;
  byte *pbVar9;
  int iVar10;
  ulonglong uVar11;
  ulonglong *puVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  uint uVar15;
  ulonglong *puVar16;
  uint uStack_888;
  uint uStack_884;
  uint local_880;
  uint local_87c;
  uint uStack_878;
  uint uStack_874;
  uint local_870;
  uint local_86c;
  uint uStack_868;
  uint uStack_864;
  uint local_860;
  uint local_85c;
  uint local_858;
  uint local_854;
  uint local_850;
  uint local_84c;
  uint uStack_848;
  uint uStack_844;
  uint uStack_840;
  uint uStack_83c;
  uint local_838;
  uint local_834;
  ulonglong *local_830 [2];
  longlong local_820;
  undefined8 local_800;
  undefined8 uStack_7f8;
  undefined *local_30;
  ulonglong local_28;
  ulonglong local_20;
  ulonglong local_18;
  undefined8 local_10;
  
  if ((*param_1 == 3) && (*(char *)((longlong)param_1 + 0x2f29) == '\x01')) {
    if ((param_1[0x5e6] != param_1[0x4c]) ||
       (((param_1[0x5e7] != param_1[0x4d] || (param_1[0x5e8] != param_1[0x4e])) ||
        (param_1[0x5e9] != param_1[0x4f])))) {
      return 0xabad1dea;
    }
    if ((param_1[0x1006] ^ 0x6e9de2b30b19f9ea) <= (param_1[0x57] ^ 0x6e9de2b30b19f9ea)) {
      return 0xdead;
    }
    if (param_1[0xafc] <= (param_1[0x1008] ^ 0x6edde0930b59ebea)) {
      param_1[0x59] = param_1[0xafc] ^ 0x6e9de2b30b19f1ea;
      param_1[0x54] = param_1[0x1003];
      param_1[0x55] = param_1[0x1004];
      param_1[0x56] = param_1[0x1005];
      param_1[0x57] = param_1[0x1006];
      param_1[0x58] = param_1[0x1007];
      param_1[0x65] = param_1[0x1009];
      param_1[0x70] = param_1[0x100a];
      return in_R0;
    }
    return 0xdeadc0de;
  }
  puVar4 = param_1 + 1;
  uVar13 = *param_1;
  if (uVar13 != 0) {
    local_830[0] = puVar4;
    puVar4 = (ulonglong *)((longlong)param_1 + param_1[0xb] + 0x286f & 0xfffffffffffffff8);
    if (uVar13 == 2) {
      local_830[1] = puVar4;
      if ((ulonglong)(byte)*puVar4 == 0xff) goto LAB_ram_00003658;
      local_830[1] = local_830[(byte)*puVar4];
      puVar4 = puVar4 + 1;
    }
    else if (uVar13 != 1) {
      ppuVar5 = local_830;
      uVar8 = uVar13;
      if (5 < uVar13) {
        ppuVar5 = local_830;
        do {
          while ((ulonglong)(byte)*puVar4 != 0xff) {
            ppuVar5[1] = local_830[(byte)*puVar4];
            puVar4 = puVar4 + 1;
            uVar11 = (ulonglong)(byte)*puVar4;
            if (uVar11 != 0xff) goto LAB_ram_00003260;
LAB_ram_00003330:
            ppuVar5[2] = puVar4;
            puVar4 = (ulonglong *)
                     ((ulonglong)((longlong)puVar4 + puVar4[10] + 0x2867) & 0xfffffffffffffff8);
            uVar11 = (ulonglong)(byte)*puVar4;
            if (uVar11 == 0xff) goto LAB_ram_00003368;
LAB_ram_000032a8:
            ppuVar5[3] = local_830[uVar11];
            puVar4 = puVar4 + 1;
            uVar11 = (ulonglong)(byte)*puVar4;
            if (uVar11 == 0xff) goto LAB_ram_00003180;
LAB_ram_000033a0:
            ppuVar5[4] = local_830[uVar11];
            puVar4 = puVar4 + 1;
            uVar11 = (ulonglong)(byte)*puVar4;
            if (uVar11 == 0xff) goto LAB_ram_000033f0;
LAB_ram_000031c0:
            ppuVar5 = ppuVar5 + 5;
            *ppuVar5 = local_830[uVar11];
            puVar4 = puVar4 + 1;
            uVar8 = uVar8 - 5;
            if (uVar8 < 6) goto LAB_ram_00003428;
          }
          ppuVar5[1] = puVar4;
          puVar4 = (ulonglong *)
                   ((ulonglong)((longlong)puVar4 + puVar4[10] + 0x2867) & 0xfffffffffffffff8);
          uVar11 = (ulonglong)(byte)*puVar4;
          if (uVar11 == 0xff) goto LAB_ram_00003330;
LAB_ram_00003260:
          ppuVar5[2] = local_830[uVar11];
          puVar4 = puVar4 + 1;
          uVar11 = (ulonglong)(byte)*puVar4;
          if (uVar11 != 0xff) goto LAB_ram_000032a8;
LAB_ram_00003368:
          ppuVar5[3] = puVar4;
          puVar4 = (ulonglong *)
                   ((ulonglong)((longlong)puVar4 + puVar4[10] + 0x2867) & 0xfffffffffffffff8);
          uVar11 = (ulonglong)(byte)*puVar4;
          if (uVar11 != 0xff) goto LAB_ram_000033a0;
LAB_ram_00003180:
          ppuVar5[4] = puVar4;
          puVar4 = (ulonglong *)
                   ((ulonglong)((longlong)puVar4 + puVar4[10] + 0x2867) & 0xfffffffffffffff8);
          uVar11 = (ulonglong)(byte)*puVar4;
          if (uVar11 != 0xff) goto LAB_ram_000031c0;
LAB_ram_000033f0:
          ppuVar5 = ppuVar5 + 5;
          *ppuVar5 = puVar4;
          puVar4 = (ulonglong *)
                   ((ulonglong)((longlong)puVar4 + puVar4[10] + 0x2867) & 0xfffffffffffffff8);
          uVar8 = uVar8 - 5;
        } while (5 < uVar8);
      }
LAB_ram_00003428:
      if (uVar8 < 4) {
        if (uVar8 == 3) {
          if ((ulonglong)(byte)*puVar4 == 0xff) {
            ppuVar5[1] = puVar4;
            puVar4 = (ulonglong *)
                     ((ulonglong)((longlong)puVar4 + puVar4[10] + 0x2867) & 0xfffffffffffffff8);
            bVar1 = (byte)*puVar4;
          }
          else {
            ppuVar5[1] = local_830[(byte)*puVar4];
            puVar4 = puVar4 + 1;
            bVar1 = (byte)*puVar4;
          }
          if ((ulonglong)bVar1 == 0xff) {
            ppuVar5[2] = puVar4;
            goto LAB_ram_00003658;
          }
          ppuVar5[2] = local_830[bVar1];
          puVar4 = puVar4 + 1;
        }
        else if (1 < uVar8) {
          if ((ulonglong)(byte)*puVar4 == 0xff) {
            ppuVar5[1] = puVar4;
            goto LAB_ram_00003658;
          }
          ppuVar5[1] = local_830[(byte)*puVar4];
          puVar4 = puVar4 + 1;
        }
      }
      else {
        uVar11 = (ulonglong)(byte)*puVar4;
        if (uVar8 == 5) {
          if (uVar11 == 0xff) {
            ppuVar5[1] = puVar4;
            puVar4 = (ulonglong *)
                     ((ulonglong)((longlong)puVar4 + puVar4[10] + 0x2867) & 0xfffffffffffffff8);
            uVar8 = (ulonglong)(byte)*puVar4;
            if (uVar8 == 0xff) goto LAB_ram_00004aa0;
LAB_ram_00003490:
            ppuVar5[2] = local_830[uVar8];
            puVar4 = puVar4 + 1;
            uVar8 = (ulonglong)(byte)*puVar4;
            if (uVar8 != 0xff) goto LAB_ram_000034d8;
LAB_ram_00004ad8:
            ppuVar5[3] = puVar4;
            puVar4 = (ulonglong *)
                     ((ulonglong)((longlong)puVar4 + puVar4[10] + 0x2867) & 0xfffffffffffffff8);
            bVar1 = (byte)*puVar4;
          }
          else {
            ppuVar5[1] = local_830[uVar11];
            puVar4 = puVar4 + 1;
            uVar8 = (ulonglong)(byte)*puVar4;
            if (uVar8 != 0xff) goto LAB_ram_00003490;
LAB_ram_00004aa0:
            ppuVar5[2] = puVar4;
            puVar4 = (ulonglong *)
                     ((ulonglong)((longlong)puVar4 + puVar4[10] + 0x2867) & 0xfffffffffffffff8);
            uVar8 = (ulonglong)(byte)*puVar4;
            if (uVar8 == 0xff) goto LAB_ram_00004ad8;
LAB_ram_000034d8:
            ppuVar5[3] = local_830[uVar8];
            puVar4 = puVar4 + 1;
            bVar1 = (byte)*puVar4;
          }
          if ((ulonglong)bVar1 != 0xff) {
            ppuVar5[4] = local_830[bVar1];
            puVar4 = puVar4 + 1;
            goto LAB_ram_00003678;
          }
          ppuVar5[4] = puVar4;
        }
        else {
          if (uVar11 == 0xff) {
            ppuVar5[1] = puVar4;
            puVar4 = (ulonglong *)
                     ((ulonglong)((longlong)puVar4 + puVar4[10] + 0x2867) & 0xfffffffffffffff8);
            uVar8 = (ulonglong)(byte)*puVar4;
            if (uVar8 == 0xff) goto LAB_ram_00004ba0;
LAB_ram_00003eb0:
            ppuVar5[2] = local_830[uVar8];
            puVar4 = puVar4 + 1;
            bVar1 = (byte)*puVar4;
          }
          else {
            ppuVar5[1] = local_830[uVar11];
            puVar4 = puVar4 + 1;
            uVar8 = (ulonglong)(byte)*puVar4;
            if (uVar8 != 0xff) goto LAB_ram_00003eb0;
LAB_ram_00004ba0:
            ppuVar5[2] = puVar4;
            puVar4 = (ulonglong *)
                     ((ulonglong)((longlong)puVar4 + puVar4[10] + 0x2867) & 0xfffffffffffffff8);
            bVar1 = (byte)*puVar4;
          }
          if ((ulonglong)bVar1 != 0xff) {
            ppuVar5[3] = local_830[bVar1];
            puVar4 = puVar4 + 1;
            goto LAB_ram_00003678;
          }
          ppuVar5[3] = puVar4;
        }
LAB_ram_00003658:
        puVar4 = (ulonglong *)
                 ((ulonglong)((longlong)puVar4 + puVar4[10] + 0x2867) & 0xfffffffffffffff8);
      }
    }
  }
LAB_ram_00003678:
  uVar8 = 0xbadc0de;
  uVar11 = *puVar4;
  if (uVar11 == 0) goto LAB_ram_00007720;
  puVar16 = puVar4 + 1;
  uVar14 = uVar11 - 1;
  uVar6 = ((ulonglong)*(byte *)((longlong)puVar16 + uVar14) ^ uVar14 >> 3 ^ 0x3a) & 0xff;
  if (0x17 < uVar6) {
    uVar6 = 0x18;
  }
  pbVar9 = (byte *)((longlong)puVar16 + uVar11);
  if (uVar6 < 0xd) {
    if (uVar6 < 6) {
      if (uVar6 < 3) {
        if (uVar6 == 0) {
          if (((*(longlong *)pbVar9 != -0x16a608d8d48b0286) ||
              (*(longlong *)(pbVar9 + 8) != 0x7a819dd33c7070c6)) ||
             ((*(longlong *)(pbVar9 + 0x10) != 0x6dd2523bce0a93a0 ||
              (bVar2 = false, *(longlong *)(pbVar9 + 0x18) != -0x2c4478dc22ab5fac)))) {
            bVar2 = true;
          }
          uVar8 = 0xdefaced;
          if (bVar2) goto LAB_ram_00007720;
          FUN_ram_00001df8(puVar16,uVar14);
          FUN_ram_00009820(&uStack_888,local_830,uVar13,puVar16,uVar14);
          uVar8 = (ulonglong)uStack_884;
          uVar15 = uStack_888;
        }
        else {
          if (uVar6 != 2) goto LAB_ram_00007888;
          uVar8 = uVar14 & 0xfffffffffffffff8;
          uVar11 = 0;
          if (uVar8 != 0) {
            lVar7 = -uVar8;
            puVar4 = puVar16;
            do {
              *puVar4 = uVar11 ^ *puVar4 ^ 0xc3ebbae2ff2fff3a;
              uVar11 = uVar11 + 0x1000100010001;
              puVar4 = puVar4 + 1;
              lVar7 = lVar7 + 8;
            } while (lVar7 != 0);
          }
          local_30 = (undefined *)0x0;
          FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
          local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
          FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
          FUN_ram_00007bd8(&local_880,local_830,uVar13,puVar16,uVar14);
          uVar8 = (ulonglong)local_87c;
          uVar15 = local_880;
        }
      }
      else if (uVar6 == 3) {
        uVar11 = 0;
        uVar8 = uVar14 & 0xfffffffffffffff8;
        if (uVar8 != 0) {
          lVar7 = -uVar8;
          puVar4 = puVar16;
          do {
            *puVar4 = uVar11 ^ *puVar4 ^ 0xc3ebbae2ff2fff3a;
            uVar11 = uVar11 + 0x1000100010001;
            puVar4 = puVar4 + 1;
            lVar7 = lVar7 + 8;
          } while (lVar7 != 0);
        }
        local_30 = (undefined *)0x0;
        FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
        local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
        FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
        FUN_ram_00008188(&uStack_878,local_830,uVar13,puVar16,uVar14);
        uVar8 = (ulonglong)uStack_874;
        uVar15 = uStack_878;
      }
      else if (uVar6 == 4) {
        uVar8 = uVar14 & 0xfffffffffffffff8;
        uVar11 = 0;
        if (uVar8 != 0) {
          lVar7 = -uVar8;
          puVar4 = puVar16;
          do {
            *puVar4 = uVar11 ^ *puVar4 ^ 0xc3ebbae2ff2fff3a;
            uVar11 = uVar11 + 0x1000100010001;
            puVar4 = puVar4 + 1;
            lVar7 = lVar7 + 8;
          } while (lVar7 != 0);
        }
        local_30 = (undefined *)0x0;
        FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
        local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
        FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
        uVar8 = 0xbadc0de;
        uVar15 = 0;
        if (8 < uVar13) {
          if (uVar14 != 0x18) goto LAB_ram_000071d0;
          if (((ulonglong)puVar16 & 7) != 0) goto LAB_ram_000071f8;
          local_30 = (undefined *)0x0;
          FUN_ram_0000dc68(&local_870,local_830,uVar13,local_800);
          uVar8 = (ulonglong)local_86c;
          uVar15 = local_870;
        }
      }
      else {
        uVar11 = 0;
        uVar8 = uVar14 & 0xfffffffffffffff8;
        if (uVar8 != 0) {
          lVar7 = -uVar8;
          puVar4 = puVar16;
          do {
            *puVar4 = uVar11 ^ *puVar4 ^ 0xc3ebbae2ff2fff3a;
            uVar11 = uVar11 + 0x1000100010001;
            puVar4 = puVar4 + 1;
            lVar7 = lVar7 + 8;
          } while (lVar7 != 0);
        }
        local_30 = (undefined *)0x0;
        FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
        local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
        FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
        FUN_ram_00011cf0(&uStack_868,local_830,uVar13,puVar16,uVar14);
        uVar8 = (ulonglong)uStack_864;
        uVar15 = uStack_868;
      }
    }
    else if (uVar6 < 9) {
      if (uVar6 == 6) {
        uVar15 = 0;
        uVar8 = uVar14 & 0xfffffffffffffff8;
        uVar11 = 0;
        if (uVar8 != 0) {
          lVar7 = -uVar8;
          puVar4 = puVar16;
          do {
            *puVar4 = uVar11 ^ *puVar4 ^ 0xc3ebbae2ff2fff3a;
            uVar11 = uVar11 + 0x1000100010001;
            puVar4 = puVar4 + 1;
            lVar7 = lVar7 + 8;
          } while (lVar7 != 0);
        }
        local_30 = (undefined *)0x0;
        FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
        local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
        FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
        puVar16 = local_830[1];
        puVar4 = local_830[0];
        uVar8 = 0xbadc0de;
        if (uVar13 == 2) {
          FUN_ram_000053c8(local_830[1] + 5,&DAT_ram_00033500,0x20,&local_30);
          iVar3 = (int)local_30;
          local_18 = puVar16[0x4e];
          local_20 = puVar16[0x4d];
          local_28 = puVar16[0x4c];
          local_30 = (undefined *)puVar16[0x4b];
          if (puVar16[0xe2] < 5) {
            local_28 = local_28 ^ 0x4a2178451bac3c7;
            local_30 = (undefined *)((ulonglong)local_30 ^ 0xfb5ce87aae443c38);
            local_20 = local_20 ^ 0x4a1178751b9c3c6;
            local_18 = local_18 ^ 0x4a0178651b8c3c5;
          }
          uVar13 = (longlong)local_30 << 0x38 | ((ulonglong)local_30 & 0xff00) << 0x28 |
                   ((ulonglong)local_30 & 0xff0000) << 0x18 |
                   ((ulonglong)local_30 & 0xff000000) << 8 | (ulonglong)local_30 >> 8 & 0xff000000 |
                   (ulonglong)local_30 >> 0x18 & 0xff0000 | (ulonglong)local_30 >> 0x28 & 0xff00 |
                   (ulonglong)local_30 >> 0x38;
          uVar8 = puVar4[1];
          uVar8 = uVar8 << 0x38 | (uVar8 & 0xff00) << 0x28 | (uVar8 & 0xff0000) << 0x18 |
                  (uVar8 & 0xff000000) << 8 | uVar8 >> 8 & 0xff000000 | uVar8 >> 0x18 & 0xff0000 |
                  uVar8 >> 0x28 & 0xff00 | uVar8 >> 0x38;
          if (uVar13 == uVar8) {
            uVar13 = local_28 << 0x38 | (local_28 & 0xff00) << 0x28 | (local_28 & 0xff0000) << 0x18
                     | (local_28 & 0xff000000) << 8 | local_28 >> 8 & 0xff000000 |
                     local_28 >> 0x18 & 0xff0000 | local_28 >> 0x28 & 0xff00 | local_28 >> 0x38;
            uVar8 = puVar4[2];
            uVar8 = uVar8 << 0x38 | (uVar8 & 0xff00) << 0x28 | (uVar8 & 0xff0000) << 0x18 |
                    (uVar8 & 0xff000000) << 8 | uVar8 >> 8 & 0xff000000 | uVar8 >> 0x18 & 0xff0000 |
                    uVar8 >> 0x28 & 0xff00 | uVar8 >> 0x38;
            if (uVar13 != uVar8) goto LAB_ram_00005578;
            uVar13 = local_20 << 0x38 | (local_20 & 0xff00) << 0x28 | (local_20 & 0xff0000) << 0x18
                     | (local_20 & 0xff000000) << 8 | local_20 >> 8 & 0xff000000 |
                     local_20 >> 0x18 & 0xff0000 | local_20 >> 0x28 & 0xff00 | local_20 >> 0x38;
            uVar8 = puVar4[3];
            uVar8 = uVar8 << 0x38 | (uVar8 & 0xff00) << 0x28 | (uVar8 & 0xff0000) << 0x18 |
                    (uVar8 & 0xff000000) << 8 | uVar8 >> 8 & 0xff000000 | uVar8 >> 0x18 & 0xff0000 |
                    uVar8 >> 0x28 & 0xff00 | uVar8 >> 0x38;
            if (uVar13 != uVar8) goto LAB_ram_00005578;
            iVar10 = 0;
            uVar13 = local_18 << 0x38 | (local_18 & 0xff00) << 0x28 | (local_18 & 0xff0000) << 0x18
                     | (local_18 & 0xff000000) << 8 | local_18 >> 8 & 0xff000000 |
                     local_18 >> 0x18 & 0xff0000 | local_18 >> 0x28 & 0xff00 | local_18 >> 0x38;
            uVar8 = puVar4[4];
            uVar8 = uVar8 << 0x38 | (uVar8 & 0xff00) << 0x28 | (uVar8 & 0xff0000) << 0x18 |
                    (uVar8 & 0xff000000) << 8 | uVar8 >> 8 & 0xff000000 | uVar8 >> 0x18 & 0xff0000 |
                    uVar8 >> 0x28 & 0xff00 | uVar8 >> 0x38;
            if (uVar13 != uVar8) goto LAB_ram_00005578;
          }
          else {
LAB_ram_00005578:
            iVar10 = -1;
            if (uVar8 <= uVar13) {
              iVar10 = 1;
            }
          }
          uVar8 = 0xabad1dea;
          if ((*(byte *)((longlong)puVar4 + 1) != 0) && (iVar10 == 0 && iVar3 == 0)) {
            puVar16[0x56] = 0x6e9de2b30b19f9ea;
            goto LAB_ram_00007068;
          }
        }
      }
      else {
        if (uVar6 != 7) {
          FUN_ram_00001df8(puVar16,uVar14);
          local_30 = &DAT_ram_000345c0;
          local_10 = 0;
          local_28 = 1;
          local_18 = 0;
          local_20 = 8;
                    /* WARNING: Subroutine does not return */
          FUN_ram_0002fba8(&local_30,&DAT_ram_000345e8);
        }
        uVar11 = 0;
        uVar8 = uVar14 & 0xfffffffffffffff8;
        if (uVar8 != 0) {
          lVar7 = -uVar8;
          puVar4 = puVar16;
          do {
            *puVar4 = uVar11 ^ *puVar4 ^ 0xc3ebbae2ff2fff3a;
            uVar11 = uVar11 + 0x1000100010001;
            puVar4 = puVar4 + 1;
            lVar7 = lVar7 + 8;
          } while (lVar7 != 0);
        }
        local_30 = (undefined *)0x0;
        FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
        local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
        FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
        FUN_ram_00012580(&local_860,local_830,uVar13,puVar16,uVar14);
        uVar8 = (ulonglong)local_85c;
        uVar15 = local_860;
      }
    }
    else if (uVar6 < 0xb) {
      if (uVar6 == 9) {
        uVar11 = 0;
        uVar8 = uVar14 & 0xfffffffffffffff8;
        if (uVar8 != 0) {
          lVar7 = -uVar8;
          puVar4 = puVar16;
          do {
            *puVar4 = uVar11 ^ *puVar4 ^ 0xc3ebbae2ff2fff3a;
            uVar11 = uVar11 + 0x1000100010001;
            puVar4 = puVar4 + 1;
            lVar7 = lVar7 + 8;
          } while (lVar7 != 0);
        }
        local_30 = (undefined *)0x0;
        FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
        local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
        FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
        FUN_ram_00012f10(&local_858,local_830,uVar13,puVar16,uVar14);
        uVar8 = (ulonglong)local_854;
        uVar15 = local_858;
      }
      else {
        uVar8 = uVar14 & 0xfffffffffffffff8;
        uVar11 = 0;
        if (uVar8 != 0) {
          lVar7 = -uVar8;
          puVar12 = puVar16;
          do {
            *puVar12 = uVar11 ^ *puVar12 ^ 0xc3ebbae2ff2fff3a;
            uVar11 = uVar11 + 0x1000100010001;
            puVar12 = puVar12 + 1;
            lVar7 = lVar7 + 8;
          } while (lVar7 != 0);
        }
        local_30 = (undefined *)0x0;
        FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
        local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
        FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
        uVar8 = 0xbadc0de;
        if ((uVar13 != 3) || (uVar14 != 0x40)) goto LAB_ram_00007448;
        uVar13 = *(ulonglong *)(local_820 + 0x58);
        local_18 = local_830[1][0x4e];
        local_20 = local_830[1][0x4d];
        local_28 = local_830[1][0x4c];
        local_30 = (undefined *)local_830[1][0x4b];
        uVar15 = 0;
        if (local_830[1][0xe2] < 5) {
          local_28 = local_28 ^ 0x4a2178451bac3c7;
          local_30 = (undefined *)((ulonglong)local_30 ^ 0xfb5ce87aae443c38);
          local_20 = local_20 ^ 0x4a1178751b9c3c6;
          local_18 = local_18 ^ 0x4a0178651b8c3c5;
        }
        if ((((local_30 != (undefined *)local_830[0][1]) || (local_28 != local_830[0][2])) ||
            (local_20 != local_830[0][3])) || (bVar2 = false, local_18 != local_830[0][4])) {
          bVar2 = true;
        }
        uVar8 = 0xabad1dea;
        if ((*(byte *)((longlong)local_830[0] + 1) != 0) && (!bVar2)) {
          uVar11 = puVar4[4];
          if (uVar11 < (local_830[1][0x56] ^ 0x6e9de2b30b19f9ea)) goto LAB_ram_00007410;
          uVar8 = 0xdeadc0de;
          if (uVar13 <= puVar4[6]) {
            uVar6 = puVar4[1];
            local_830[1][0x54] = puVar4[2] ^ 0x46a912eb23798bd9;
            local_830[1][0x53] = uVar6 ^ 0xb957ed15dc877426;
            uVar6 = puVar4[3];
            local_830[1][0x56] = uVar11 ^ 0x6e9de2b30b19f9ea;
            local_830[1][0x55] = uVar6 ^ 0x6e9de2b30b19f9ea;
            uVar11 = puVar4[5];
            local_830[1][0x58] = uVar13 ^ 0x6e9de2b30b19f1ea;
            local_830[1][0x57] = uVar11 ^ 0x6e9de2b30b19f1ea;
            local_830[1][100] = puVar4[7] ^ 0xd3198133b7c1776c;
            local_830[1][0x6f] = puVar4[8] ^ 0x504156a22548f8dd;
            goto LAB_ram_00007068;
          }
        }
      }
    }
    else {
      if (uVar6 != 0xb) {
        FUN_ram_00001df8(puVar16,uVar14);
        local_30 = &DAT_ram_000345c0;
        local_10 = 0;
        local_28 = 1;
        local_18 = 0;
        local_20 = 8;
                    /* WARNING: Subroutine does not return */
        FUN_ram_0002fba8(&local_30,&DAT_ram_00034600);
      }
      uVar8 = uVar14 & 0xfffffffffffffff8;
      uVar11 = 0;
      if (uVar8 != 0) {
        lVar7 = -uVar8;
        puVar4 = puVar16;
        do {
          *puVar4 = uVar11 ^ *puVar4 ^ 0xc3ebbae2ff2fff3a;
          uVar11 = uVar11 + 0x1000100010001;
          puVar4 = puVar4 + 1;
          lVar7 = lVar7 + 8;
        } while (lVar7 != 0);
      }
      local_30 = (undefined *)0x0;
      FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
      local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
      FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
      uVar8 = 0xbadc0de;
      if (uVar13 != 2) goto LAB_ram_00007448;
      if (uVar14 != 8) goto LAB_ram_000071d0;
      uVar15 = 0;
      if (((ulonglong)puVar16 & 7) != 0) goto LAB_ram_000071f8;
      local_18 = local_830[1][0x4e];
      local_20 = local_830[1][0x4d];
      local_28 = local_830[1][0x4c];
      local_30 = (undefined *)local_830[1][0x4b];
      if (local_830[1][0xe2] < 5) {
        local_28 = local_28 ^ 0x4a2178451bac3c7;
        local_30 = (undefined *)((ulonglong)local_30 ^ 0xfb5ce87aae443c38);
        local_20 = local_20 ^ 0x4a1178751b9c3c6;
        local_18 = local_18 ^ 0x4a0178651b8c3c5;
      }
      if ((((local_30 != (undefined *)local_830[0][1]) || (local_28 != local_830[0][2])) ||
          (local_20 != local_830[0][3])) || (bVar2 = false, local_18 != local_830[0][4])) {
        bVar2 = true;
      }
      uVar8 = 0xabad1dea;
      if ((*(byte *)((longlong)local_830[0] + 1) != 0) && (!bVar2)) {
        local_830[1][0x6f] = *puVar16 ^ 0x504156a22548f8dd;
        uVar8 = 0xbad4;
        if (local_830[1][0xe2] == 2) {
          local_830[1][0xe2] = 4;
          goto LAB_ram_00007068;
        }
      }
    }
  }
  else if (uVar6 < 0x13) {
    if (uVar6 < 0x10) {
      if (uVar6 == 0xd) {
        uVar15 = 0;
        if ((uVar13 == 3) && (uVar14 == 0x40)) {
          uVar13 = *(ulonglong *)(local_820 + 0x58);
          local_18 = local_830[1][0x4e];
          local_20 = local_830[1][0x4d];
          local_28 = local_830[1][0x4c];
          local_30 = (undefined *)local_830[1][0x4b];
          if (local_830[1][0xe2] < 5) {
            local_28 = local_28 ^ 0x4a2178451bac3c7;
            local_30 = (undefined *)((ulonglong)local_30 ^ 0xfb5ce87aae443c38);
            local_20 = local_20 ^ 0x4a1178751b9c3c6;
            local_18 = local_18 ^ 0x4a0178651b8c3c5;
          }
          if (((local_30 != (undefined *)param_1[2]) || (local_28 != param_1[3])) ||
             ((local_20 != param_1[4] || (bVar2 = false, local_18 != param_1[5])))) {
            bVar2 = true;
          }
          uVar8 = 0xabad1dea;
          if ((*(char *)((longlong)param_1 + 9) != '\0') && (!bVar2)) {
            uVar11 = puVar4[4];
            if ((uVar11 ^ 0x6e9de2b30b19f9ea) < (local_830[1][0x56] ^ 0x6e9de2b30b19f9ea)) {
LAB_ram_00007410:
              uVar15 = 0;
              uVar8 = 0xdead;
            }
            else {
              uVar8 = puVar4[6];
              puVar4[6] = uVar8 ^ 0x6edde0930b59ebea;
              if (uVar13 <= (uVar8 ^ 0x6edde0930b59ebea)) {
                uVar6 = puVar4[1];
                uVar8 = puVar4[2];
                local_830[1][0x54] = uVar8;
                local_830[1][0x53] = uVar6;
                uVar6 = puVar4[3];
                local_830[1][0x56] = uVar11;
                local_830[1][0x55] = uVar6;
                local_830[1][0x57] = puVar4[5];
                local_830[1][100] = puVar4[7];
                uVar11 = puVar4[8];
                local_830[1][0x58] = uVar13 ^ 0x6e9de2b30b19f1ea;
                local_830[1][0x6f] = uVar11;
                goto LAB_ram_00007068;
              }
              uVar8 = 0xdeadc0de;
            }
          }
        }
      }
      else if (uVar6 == 0xe) {
        if ((((*(longlong *)pbVar9 != -0x16a608d8d48b0286) ||
             (*(longlong *)(pbVar9 + 8) != 0x7a819dd33c7070c6)) ||
            (*(longlong *)(pbVar9 + 0x10) != 0x6dd2523bce0a93a0)) ||
           (bVar2 = false, *(longlong *)(pbVar9 + 0x18) != -0x2c4478dc22ab5fac)) {
          bVar2 = true;
        }
        uVar8 = 0xdefaced;
        if (bVar2) goto LAB_ram_00007720;
        FUN_ram_00001df8(puVar16,uVar14);
        FUN_ram_0000a860(&local_850,local_830,uVar13,puVar16,uVar14);
        uVar8 = (ulonglong)local_84c;
        uVar15 = local_850;
      }
      else {
        uVar8 = uVar14 & 0xfffffffffffffff8;
        uVar11 = 0;
        if (uVar8 != 0) {
          lVar7 = -uVar8;
          puVar12 = puVar16;
          do {
            *puVar12 = uVar11 ^ *puVar12 ^ 0xc3ebbae2ff2fff3a;
            uVar11 = uVar11 + 0x1000100010001;
            puVar12 = puVar12 + 1;
            lVar7 = lVar7 + 8;
          } while (lVar7 != 0);
        }
        local_30 = (undefined *)0x0;
        FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
        local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
        FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
        uVar8 = 0xbadc0de1;
        if (uVar13 < 0xc) goto LAB_ram_00007448;
        uVar8 = 0xbadc0ded;
        uVar15 = 0;
        if (uVar14 == 0x18) {
          if (((ulonglong)puVar16 & 7) != 0) goto LAB_ram_000071f8;
          FUN_ram_0000f9f8(&uStack_848,local_830,uVar13,puVar4[1]);
          uVar8 = (ulonglong)uStack_844;
          uVar15 = uStack_848;
        }
      }
    }
    else if (uVar6 == 0x10) {
      uVar8 = uVar14 & 0xfffffffffffffff8;
      uVar11 = 0;
      if (uVar8 != 0) {
        lVar7 = -uVar8;
        puVar12 = puVar16;
        do {
          *puVar12 = uVar11 ^ *puVar12 ^ 0xc3ebbae2ff2fff3a;
          uVar11 = uVar11 + 0x1000100010001;
          puVar12 = puVar12 + 1;
          lVar7 = lVar7 + 8;
        } while (lVar7 != 0);
      }
      local_30 = (undefined *)0x0;
      FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
      local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
      FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
      uVar8 = 0xbadc0de;
      uVar15 = 0;
      if (9 < uVar13) {
        if (uVar14 != 0x18) goto LAB_ram_000071d0;
        if (((ulonglong)puVar16 & 7) != 0) goto LAB_ram_000071f8;
        local_20 = 3;
        local_28 = 3;
        if (*(byte *)((longlong)puVar4 + 0x1a) != 1) {
          local_20 = 0;
          local_28 = (ulonglong)*(byte *)((longlong)puVar4 + 0x19);
        }
        local_18 = (ulonglong)*(byte *)((longlong)puVar4 + 0x1a);
        local_30 = (undefined *)0x1;
        FUN_ram_0000dc68(&uStack_840,local_830,uVar13,uStack_7f8);
        uVar8 = (ulonglong)uStack_83c;
        uVar15 = uStack_840;
      }
    }
    else {
      if (uVar6 != 0x12) {
        FUN_ram_0002fbd8("internal error: entered unreachable code",0x28,&DAT_ram_00034618);
LAB_ram_00007888:
        FUN_ram_00001df8(puVar16,uVar14);
        local_30 = &DAT_ram_000345c0;
        local_10 = 0;
        local_28 = 1;
        local_18 = 0;
        local_20 = 8;
                    /* WARNING: Subroutine does not return */
        FUN_ram_0002fba8(&local_30,&DAT_ram_000345d0);
      }
      uVar15 = 0;
      uVar8 = uVar14 & 0xfffffffffffffff8;
      uVar11 = 0;
      if (uVar8 != 0) {
        lVar7 = -uVar8;
        puVar4 = puVar16;
        do {
          *puVar4 = uVar11 ^ *puVar4 ^ 0xc3ebbae2ff2fff3a;
          uVar11 = uVar11 + 0x1000100010001;
          puVar4 = puVar4 + 1;
          lVar7 = lVar7 + 8;
        } while (lVar7 != 0);
      }
      local_30 = (undefined *)0x0;
      FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
      local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
      FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
      uVar8 = 0xbadc0de;
      if (uVar13 == 2) {
        local_18 = local_830[1][0x4e];
        local_20 = local_830[1][0x4d];
        local_28 = local_830[1][0x4c];
        local_30 = (undefined *)local_830[1][0x4b];
        if (local_830[1][0xe2] < 5) {
          local_28 = local_28 ^ 0x4a2178451bac3c7;
          local_30 = (undefined *)((ulonglong)local_30 ^ 0xfb5ce87aae443c38);
          local_20 = local_20 ^ 0x4a1178751b9c3c6;
          local_18 = local_18 ^ 0x4a0178651b8c3c5;
        }
        if (((local_30 != (undefined *)local_830[0][1]) || (local_28 != local_830[0][2])) ||
           ((local_20 != local_830[0][3] || (bVar2 = false, local_18 != local_830[0][4])))) {
          bVar2 = true;
        }
        uVar8 = 0xabad1dea;
        if (((*(byte *)((longlong)local_830[0] + 1) != 0) && (!bVar2)) &&
           (uVar8 = 0xbad4, local_830[1][0xe2] == 4)) {
          local_830[1][0x4b] = local_830[1][0x4b] ^ 0xfb5ce87aae443c38;
          local_830[1][0x4c] = local_830[1][0x4c] ^ 0x4a2178451bac3c7;
          local_830[1][0x4d] = local_830[1][0x4d] ^ 0x4a1178751b9c3c6;
          local_830[1][0x4e] = local_830[1][0x4e] ^ 0x4a0178651b8c3c5;
          local_830[1][0xe2] = 5;
          goto LAB_ram_00007068;
        }
      }
    }
  }
  else if (uVar6 < 0x16) {
    if (uVar6 == 0x13) {
      uVar8 = uVar14 & 0xfffffffffffffff8;
      uVar11 = 0;
      if (uVar8 != 0) {
        lVar7 = -uVar8;
        puVar12 = puVar16;
        do {
          *puVar12 = uVar11 ^ *puVar12 ^ 0xc3ebbae2ff2fff3a;
          uVar11 = uVar11 + 0x1000100010001;
          puVar12 = puVar12 + 1;
          lVar7 = lVar7 + 8;
        } while (lVar7 != 0);
      }
      local_30 = (undefined *)0x0;
      FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
      local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
      FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
      uVar8 = 0xbadc0de;
      if (uVar13 == 2) {
        if (uVar14 != 0x10) {
LAB_ram_000071d0:
                    /* WARNING: Subroutine does not return */
          FUN_ram_000011b0(&DAT_ram_000337df,10,2);
        }
        uVar15 = 0;
        if (((ulonglong)puVar16 & 7) != 0) {
LAB_ram_000071f8:
                    /* WARNING: Subroutine does not return */
          FUN_ram_000011b0(&DAT_ram_000337df,10,0);
        }
        local_18 = local_830[1][0x4e];
        local_20 = local_830[1][0x4d];
        local_28 = local_830[1][0x4c];
        local_30 = (undefined *)local_830[1][0x4b];
        if (local_830[1][0xe2] < 5) {
          local_28 = local_28 ^ 0x4a2178451bac3c7;
          local_30 = (undefined *)((ulonglong)local_30 ^ 0xfb5ce87aae443c38);
          local_20 = local_20 ^ 0x4a1178751b9c3c6;
          local_18 = local_18 ^ 0x4a0178651b8c3c5;
        }
        if (((local_30 != (undefined *)local_830[0][1]) || (local_28 != local_830[0][2])) ||
           ((local_20 != local_830[0][3] || (bVar2 = false, local_18 != local_830[0][4])))) {
          bVar2 = true;
        }
        uVar8 = 0xabad1dea;
        if ((*(byte *)((longlong)local_830[0] + 1) != 0) && (!bVar2)) {
          local_830[1][0x70] = puVar4[1] ^ 0x35f72d643d3464eb;
          local_830[1][0x71] = puVar4[2] ^ 0x9578e14d1d0d9c4e;
          uVar8 = 0xbad4;
          if (local_830[1][0xe2] == 5) {
            local_830[1][0xe2] = 6;
            goto LAB_ram_00007068;
          }
        }
      }
      else {
LAB_ram_00007448:
        uVar15 = 0;
      }
    }
    else if (uVar6 == 0x14) {
      uVar11 = 0;
      uVar8 = uVar14 & 0xfffffffffffffff8;
      if (uVar8 != 0) {
        lVar7 = -uVar8;
        puVar4 = puVar16;
        do {
          *puVar4 = uVar11 ^ *puVar4 ^ 0xc3ebbae2ff2fff3a;
          uVar11 = uVar11 + 0x1000100010001;
          puVar4 = puVar4 + 1;
          lVar7 = lVar7 + 8;
        } while (lVar7 != 0);
      }
      local_30 = (undefined *)0x0;
      FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
      local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
      FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
      FUN_ram_00011490(&local_838,local_830,uVar13,puVar16,uVar14);
      uVar8 = (ulonglong)local_834;
      uVar15 = local_838;
    }
    else {
      uVar8 = uVar14 & 0xfffffffffffffff8;
      uVar11 = 0;
      if (uVar8 != 0) {
        lVar7 = -uVar8;
        puVar12 = puVar16;
        do {
          *puVar12 = uVar11 ^ *puVar12 ^ 0xc3ebbae2ff2fff3a;
          uVar11 = uVar11 + 0x1000100010001;
          puVar12 = puVar12 + 1;
          lVar7 = lVar7 + 8;
        } while (lVar7 != 0);
      }
      local_30 = (undefined *)0x0;
      FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
      local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
      FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
      puVar12 = local_830[1];
      uVar8 = 0xbadc0de;
      if (uVar13 != 2) goto LAB_ram_00007448;
      if (uVar14 != 0x50) goto LAB_ram_000071d0;
      uVar15 = 0;
      if (((ulonglong)puVar16 & 7) != 0) goto LAB_ram_000071f8;
      local_18 = local_830[1][0x4e];
      local_20 = local_830[1][0x4d];
      local_28 = local_830[1][0x4c];
      local_30 = (undefined *)local_830[1][0x4b];
      uVar13 = local_830[1][0xe2];
      if (uVar13 < 5) {
        local_28 = local_28 ^ 0x4a2178451bac3c7;
        local_30 = (undefined *)((ulonglong)local_30 ^ 0xfb5ce87aae443c38);
        local_20 = local_20 ^ 0x4a1178751b9c3c6;
        local_18 = local_18 ^ 0x4a0178651b8c3c5;
      }
      if ((((local_30 != (undefined *)local_830[0][1]) || (local_28 != local_830[0][2])) ||
          (local_20 != local_830[0][3])) || (bVar2 = false, local_18 != local_830[0][4])) {
        bVar2 = true;
      }
      uVar8 = 0xabad1dea;
      if ((*(byte *)((longlong)local_830[0] + 1) != 0) && (!bVar2)) {
        uVar8 = 0xbadc0df1;
        if (puVar4[2] < 0x7a121) {
          if ((((puVar4[1] < puVar4[3]) || (puVar4[2] < puVar4[4])) ||
              ((puVar4[3] < puVar4[5] ||
               (((puVar4[4] < puVar4[6] || (puVar4[5] < puVar4[7])) || (puVar4[6] < puVar4[8]))))))
             || ((puVar4[7] < puVar4[9] || (puVar4[8] < puVar4[10])))) {
            uVar8 = 0xbadc0df0;
          }
          else if (puVar4[10] == 0) {
            FUN_ram_00031b28(local_830[1] + 0x72,puVar16,0x50);
            uVar8 = puVar12[0x73] ^ 0x9aa8843b60a9bf;
            puVar12[0x72] = puVar12[0x72] ^ 0xff64577ac49fae40;
            puVar12[0x73] = uVar8;
            puVar12[0x74] = puVar12[0x74] ^ 0x99a8873b61a9be;
            puVar12[0x75] = puVar12[0x75] ^ 0x98a8863b62a9bd;
            puVar12[0x76] = puVar12[0x76] ^ 0x9fa8813b63a9bc;
            puVar12[0x77] = puVar12[0x77] ^ 0x9ea8803b64a9bb;
            puVar12[0x78] = puVar12[0x78] ^ 0x9da8833b65a9ba;
            puVar12[0x79] = puVar12[0x79] ^ 0x9ca8823b66a9b9;
            puVar12[0x7a] = puVar12[0x7a] ^ 0x93a88d3b67a9b8;
            puVar12[0x7b] = puVar12[0x7b] ^ 0x92a88c3b68a9b7;
            if (uVar13 == 6) {
              puVar12[0xe2] = 7;
            }
            else if (uVar13 < 6) {
              uVar8 = 0xbad4;
              goto LAB_ram_00007448;
            }
            goto LAB_ram_00007068;
          }
        }
      }
    }
  }
  else if (uVar6 == 0x16) {
    uVar8 = uVar14 & 0xfffffffffffffff8;
    uVar11 = 0;
    if (uVar8 != 0) {
      lVar7 = -uVar8;
      puVar12 = puVar16;
      do {
        *puVar12 = uVar11 ^ *puVar12 ^ 0xc3ebbae2ff2fff3a;
        uVar11 = uVar11 + 0x1000100010001;
        puVar12 = puVar12 + 1;
        lVar7 = lVar7 + 8;
      } while (lVar7 != 0);
    }
    local_30 = (undefined *)0x0;
    FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
    local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
    FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
    uVar8 = 0xbadc0de;
    if (uVar13 != 2) goto LAB_ram_00007448;
    if (uVar14 != 0x30) goto LAB_ram_000071d0;
    uVar15 = 0;
    if (((ulonglong)puVar16 & 7) != 0) goto LAB_ram_000071f8;
    local_18 = local_830[1][0x4e];
    local_20 = local_830[1][0x4d];
    local_28 = local_830[1][0x4c];
    local_30 = (undefined *)local_830[1][0x4b];
    uVar13 = local_830[1][0xe2];
    if (uVar13 < 5) {
      local_28 = local_28 ^ 0x4a2178451bac3c7;
      local_30 = (undefined *)((ulonglong)local_30 ^ 0xfb5ce87aae443c38);
      local_20 = local_20 ^ 0x4a1178751b9c3c6;
      local_18 = local_18 ^ 0x4a0178651b8c3c5;
    }
    if (((local_30 != (undefined *)local_830[0][1]) || (local_28 != local_830[0][2])) ||
       ((local_20 != local_830[0][3] || (bVar2 = false, local_18 != local_830[0][4])))) {
      bVar2 = true;
    }
    uVar8 = 0xabad1dea;
    if (((*(byte *)((longlong)local_830[0] + 1) != 0) && (!bVar2)) && (uVar8 = 0xbad4, 6 < uVar13))
    {
      if ((((*puVar16 != 0) || (puVar4[2] != 0)) || (puVar4[3] != 0)) || (puVar4[4] != 0)) {
        uVar11 = puVar4[3];
        local_830[1][0x7e] = uVar11;
        uVar6 = puVar4[2];
        local_830[1][0x7d] = uVar6;
        uVar8 = *puVar16;
        local_830[1][0x7c] = uVar8;
        uVar8 = uVar8 ^ 0xfb5ce87aae443c38;
        uVar14 = puVar4[4];
        local_830[1][0x7c] = uVar8;
        local_830[1][0x7d] = uVar6 ^ 0x4a2178451bac3c7;
        local_830[1][0x7e] = uVar11 ^ 0x4a1178751b9c3c6;
        local_830[1][0x7f] = uVar14;
        local_830[1][0x7f] = uVar14 ^ 0x4a0178651b8c3c5;
        local_830[1][0x80] = puVar4[5] ^ 0xcf44133cb352d91c;
        if ((byte)puVar4[6] != 0) {
          pbVar9 = (byte *)((longlong)local_830[1] + 0x289);
          pbVar9[0] = 0;
          pbVar9[1] = 0;
          pbVar9[2] = 0;
          pbVar9[3] = 0;
          *(byte *)(local_830[1] + 0x51) = 2;
          pbVar9 = (byte *)((longlong)local_830[1] + 0x28c);
          pbVar9[0] = 0;
          pbVar9[1] = 0;
          pbVar9[2] = 0;
          pbVar9[3] = 0;
          local_830[1][0x51] = local_830[1][0x51] ^ 0xf539f2cf9513d4a1;
        }
        if (uVar13 == 7) {
          local_830[1][0xe2] = 8;
        }
        goto LAB_ram_00007068;
      }
LAB_ram_00007190:
      uVar15 = 0;
      uVar8 = 0xbadc0df2;
    }
  }
  else {
    if (uVar6 != 0x17) goto LAB_ram_00007720;
    uVar8 = uVar14 & 0xfffffffffffffff8;
    uVar11 = 0;
    if (uVar8 != 0) {
      lVar7 = -uVar8;
      puVar4 = puVar16;
      do {
        *puVar4 = uVar11 ^ *puVar4 ^ 0xc3ebbae2ff2fff3a;
        uVar11 = uVar11 + 0x1000100010001;
        puVar4 = puVar4 + 1;
        lVar7 = lVar7 + 8;
      } while (lVar7 != 0);
    }
    local_30 = (undefined *)0x0;
    FUN_ram_00031b28(&local_30,(byte *)((longlong)puVar16 + uVar8),uVar14 & 7);
    local_30 = (undefined *)(uVar11 ^ (ulonglong)local_30 ^ 0xc3ebbae2ff2fff3a);
    FUN_ram_00031b28((byte *)((longlong)puVar16 + uVar8),&local_30,uVar14 & 7);
    uVar8 = 0xbadc0de;
    if (uVar13 != 2) goto LAB_ram_00007448;
    if (uVar14 != 8) goto LAB_ram_000071d0;
    local_18 = local_830[1][0x4e];
    local_20 = local_830[1][0x4d];
    local_28 = local_830[1][0x4c];
    local_30 = (undefined *)local_830[1][0x4b];
    uVar15 = 0;
    if (local_830[1][0xe2] < 5) {
      local_28 = local_28 ^ 0x4a2178451bac3c7;
      local_30 = (undefined *)((ulonglong)local_30 ^ 0xfb5ce87aae443c38);
      local_20 = local_20 ^ 0x4a1178751b9c3c6;
      local_18 = local_18 ^ 0x4a0178651b8c3c5;
    }
    if (((local_30 != (undefined *)param_1[2]) || (local_28 != param_1[3])) ||
       ((local_20 != param_1[4] || (bVar2 = false, local_18 != param_1[5])))) {
      bVar2 = true;
    }
    uVar8 = 0xabad1dea;
    if ((*(char *)((longlong)param_1 + 9) == '\0') || (bVar2)) goto LAB_ram_00007450;
    bVar1 = (byte)*puVar16;
    if (1 < bVar1) {
      uVar8 = 0xbadc0de9;
      if ((bVar1 != 2) || (uVar8 = 0xbad4, local_830[1][0xe2] < 8)) goto LAB_ram_00007450;
      uVar8 = local_830[1][0x7d] ^ 0x4a2178451bac3c7;
      if ((local_830[1][0x7c] == 0xfb5ce87aae443c38) &&
         (((uVar8 == 0 && (local_830[1][0x7e] == 0x4a1178751b9c3c6)) &&
          (local_830[1][0x7f] == 0x4a0178651b8c3c5)))) goto LAB_ram_00007190;
    }
    *(byte *)(local_830[1] + 0x51) = bVar1;
    pbVar9 = (byte *)((longlong)local_830[1] + 0x289);
    pbVar9[0] = 0;
    pbVar9[1] = 0;
    pbVar9[2] = 0;
    pbVar9[3] = 0;
    pbVar9 = (byte *)((longlong)local_830[1] + 0x28c);
    pbVar9[0] = 0;
    pbVar9[1] = 0;
    pbVar9[2] = 0;
    pbVar9[3] = 0;
    local_830[1][0x51] = local_830[1][0x51] ^ 0xf539f2cf9513d4a1;
LAB_ram_00007068:
    uVar15 = 0x1a;
  }
LAB_ram_00007450:
  if (0xc < uVar15) {
    if (0x13 < uVar15) {
      if (uVar15 < 0x17) {
        if (uVar15 == 0x14) {
          return 0x1500000000;
        }
        if (uVar15 == 0x15) {
          return 0x1600000000;
        }
        return 0x1700000000;
      }
      if (0x18 < uVar15) {
        if (uVar15 != 0x19) {
          return 0;
        }
        return 0x1a00000000;
      }
      if (uVar15 == 0x17) {
        return 0x1800000000;
      }
      return 0x1900000000;
    }
    if (uVar15 < 0x10) {
      if (uVar15 == 0xd) {
        return 0xe00000000;
      }
      if (uVar15 == 0xe) {
        return 0xf00000000;
      }
      return 0x1000000000;
    }
    if (0x11 < uVar15) {
      if (uVar15 == 0x12) {
        return 0x1300000000;
      }
      return 0x1400000000;
    }
    if (uVar15 == 0x10) {
      return 0x1100000000;
    }
    return 0x1200000000;
  }
  if (5 < uVar15) {
    if (uVar15 < 9) {
      if (uVar15 == 6) {
        return 0x700000000;
      }
      if (uVar15 == 7) {
        return 0x800000000;
      }
      return 0x900000000;
    }
    if (10 < uVar15) {
      if (uVar15 == 0xb) {
        return 0xc00000000;
      }
      return 0xd00000000;
    }
    if (uVar15 == 9) {
      return 0xa00000000;
    }
    return 0xb00000000;
  }
  if (2 < uVar15) {
    if (uVar15 == 3) {
      return 0x400000000;
    }
    if (uVar15 == 4) {
      return 0x500000000;
    }
    return 0x600000000;
  }
  if (uVar15 != 0) {
    if (uVar15 == 1) {
      return 0x200000000;
    }
    return 0x300000000;
  }
  if ((uVar8 & 0xffffffff) == 0) {
    return 0x100000000;
  }
LAB_ram_00007720:
  return uVar8 & 0xffffffff;
}

// Function: FUN_ram_000053c8
/* WARNING: Removing unreachable block (ram,0x000077c8) */

undefined8 FUN_ram_000053c8(void)

{
  int iVar1;
  undefined8 uVar2;
  ulonglong uVar3;
  int iVar4;
  ulonglong uVar5;
  longlong unaff_R6;
  longlong unaff_R7;
  uint unaff_R9;
  undefined8 local_30;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_18;
  
  iVar1 = (int)local_30;
  FUN_ram_000053c8();
  local_18 = *(ulonglong *)(unaff_R6 + 0x270);
  local_20 = *(ulonglong *)(unaff_R6 + 0x268);
  local_28 = *(ulonglong *)(unaff_R6 + 0x260);
  local_30 = *(ulonglong *)(unaff_R6 + 600);
  if (*(ulonglong *)(unaff_R6 + 0x710) < 5) {
    local_28 = local_28 ^ 0x4a2178451bac3c7;
    local_30 = local_30 ^ 0xfb5ce87aae443c38;
    local_20 = local_20 ^ 0x4a1178751b9c3c6;
    local_18 = local_18 ^ 0x4a0178651b8c3c5;
  }
  uVar3 = local_30 << 0x38 | (local_30 & 0xff00) << 0x28 | (local_30 & 0xff0000) << 0x18 |
          (local_30 & 0xff000000) << 8 | local_30 >> 8 & 0xff000000 | local_30 >> 0x18 & 0xff0000 |
          local_30 >> 0x28 & 0xff00 | local_30 >> 0x38;
  uVar5 = *(ulonglong *)(unaff_R7 + 8);
  uVar5 = uVar5 << 0x38 | (uVar5 & 0xff00) << 0x28 | (uVar5 & 0xff0000) << 0x18 |
          (uVar5 & 0xff000000) << 8 | uVar5 >> 8 & 0xff000000 | uVar5 >> 0x18 & 0xff0000 |
          uVar5 >> 0x28 & 0xff00 | uVar5 >> 0x38;
  if (uVar3 == uVar5) {
    uVar3 = local_28 << 0x38 | (local_28 & 0xff00) << 0x28 | (local_28 & 0xff0000) << 0x18 |
            (local_28 & 0xff000000) << 8 | local_28 >> 8 & 0xff000000 | local_28 >> 0x18 & 0xff0000
            | local_28 >> 0x28 & 0xff00 | local_28 >> 0x38;
    uVar5 = *(ulonglong *)(unaff_R7 + 0x10);
    uVar5 = uVar5 << 0x38 | (uVar5 & 0xff00) << 0x28 | (uVar5 & 0xff0000) << 0x18 |
            (uVar5 & 0xff000000) << 8 | uVar5 >> 8 & 0xff000000 | uVar5 >> 0x18 & 0xff0000 |
            uVar5 >> 0x28 & 0xff00 | uVar5 >> 0x38;
    if (uVar3 == uVar5) {
      uVar3 = local_20 << 0x38 | (local_20 & 0xff00) << 0x28 | (local_20 & 0xff0000) << 0x18 |
              (local_20 & 0xff000000) << 8 | local_20 >> 8 & 0xff000000 |
              local_20 >> 0x18 & 0xff0000 | local_20 >> 0x28 & 0xff00 | local_20 >> 0x38;
      uVar5 = *(ulonglong *)(unaff_R7 + 0x18);
      uVar5 = uVar5 << 0x38 | (uVar5 & 0xff00) << 0x28 | (uVar5 & 0xff0000) << 0x18 |
              (uVar5 & 0xff000000) << 8 | uVar5 >> 8 & 0xff000000 | uVar5 >> 0x18 & 0xff0000 |
              uVar5 >> 0x28 & 0xff00 | uVar5 >> 0x38;
      if (uVar3 == uVar5) {
        iVar4 = 0;
        uVar3 = local_18 << 0x38 | (local_18 & 0xff00) << 0x28 | (local_18 & 0xff0000) << 0x18 |
                (local_18 & 0xff000000) << 8 | local_18 >> 8 & 0xff000000 |
                local_18 >> 0x18 & 0xff0000 | local_18 >> 0x28 & 0xff00 | local_18 >> 0x38;
        uVar5 = *(ulonglong *)(unaff_R7 + 0x20);
        uVar5 = uVar5 << 0x38 | (uVar5 & 0xff00) << 0x28 | (uVar5 & 0xff0000) << 0x18 |
                (uVar5 & 0xff000000) << 8 | uVar5 >> 8 & 0xff000000 | uVar5 >> 0x18 & 0xff0000 |
                uVar5 >> 0x28 & 0xff00 | uVar5 >> 0x38;
        if (uVar3 == uVar5) goto LAB_ram_00005590;
      }
    }
  }
  iVar4 = -1;
  if (uVar5 <= uVar3) {
    iVar4 = 1;
  }
LAB_ram_00005590:
  if ((*(char *)(unaff_R7 + 1) != '\0') && (iVar4 == 0 && iVar1 == 0)) {
    *(undefined8 *)(unaff_R6 + 0x2b0) = 0x6e9de2b30b19f9ea;
    unaff_R9 = 0x1a;
  }
  if (unaff_R9 < 0xd) {
    if (unaff_R9 < 6) {
      if (unaff_R9 < 3) {
        if (unaff_R9 == 0) {
          uVar2 = 0xabad1dea;
        }
        else if (unaff_R9 == 1) {
          uVar2 = 0x200000000;
        }
        else {
          uVar2 = 0x300000000;
        }
      }
      else if (unaff_R9 == 3) {
        uVar2 = 0x400000000;
      }
      else if (unaff_R9 == 4) {
        uVar2 = 0x500000000;
      }
      else {
        uVar2 = 0x600000000;
      }
    }
    else if (unaff_R9 < 9) {
      if (unaff_R9 == 6) {
        uVar2 = 0x700000000;
      }
      else if (unaff_R9 == 7) {
        uVar2 = 0x800000000;
      }
      else {
        uVar2 = 0x900000000;
      }
    }
    else if (unaff_R9 < 0xb) {
      if (unaff_R9 == 9) {
        uVar2 = 0xa00000000;
      }
      else {
        uVar2 = 0xb00000000;
      }
    }
    else if (unaff_R9 == 0xb) {
      uVar2 = 0xc00000000;
    }
    else {
      uVar2 = 0xd00000000;
    }
  }
  else if (unaff_R9 < 0x14) {
    if (unaff_R9 < 0x10) {
      if (unaff_R9 == 0xd) {
        uVar2 = 0xe00000000;
      }
      else if (unaff_R9 == 0xe) {
        uVar2 = 0xf00000000;
      }
      else {
        uVar2 = 0x1000000000;
      }
    }
    else if (unaff_R9 < 0x12) {
      if (unaff_R9 == 0x10) {
        uVar2 = 0x1100000000;
      }
      else {
        uVar2 = 0x1200000000;
      }
    }
    else if (unaff_R9 == 0x12) {
      uVar2 = 0x1300000000;
    }
    else {
      uVar2 = 0x1400000000;
    }
  }
  else if (unaff_R9 < 0x17) {
    if (unaff_R9 == 0x14) {
      uVar2 = 0x1500000000;
    }
    else if (unaff_R9 == 0x15) {
      uVar2 = 0x1600000000;
    }
    else {
      uVar2 = 0x1700000000;
    }
  }
  else if (unaff_R9 < 0x19) {
    if (unaff_R9 == 0x17) {
      uVar2 = 0x1800000000;
    }
    else {
      uVar2 = 0x1900000000;
    }
  }
  else {
    uVar2 = 0;
    if (unaff_R9 == 0x19) {
      uVar2 = 0x1a00000000;
    }
  }
  return uVar2;
}

// Function: FUN_ram_00007978
void FUN_ram_00007978(undefined8 *param_1,char param_2,longlong *param_3,longlong param_4,
                     longlong *param_5)

{
  bool bVar1;
  longlong lVar2;
  ulonglong uVar3;
  
  if ((((*param_5 != 0x77c2575f37eddd1b) || (param_5[1] != 0x2d6e7a2be59cf048)) ||
      (param_5[2] != 0x7ea41dd6046c6fc4)) || (bVar1 = false, param_5[3] != 0x31797eed4f7e7455)) {
    bVar1 = true;
  }
  if ((!bVar1) || (param_2 == '\0')) {
    *(undefined4 *)param_1 = 0;
    param_1[1] = 0;
    return;
  }
  if (param_2 == '\x01') {
    *(undefined4 *)(param_1 + 1) = 0x1ced;
    *param_1 = 1;
    return;
  }
  if (param_3 != (longlong *)0x0) {
    lVar2 = *param_3;
    if (((*(ulonglong *)(lVar2 + 8) != (*(ulonglong *)(param_4 + 0x388) ^ 0xfb5ce87aae443c38)) ||
        (*(ulonglong *)(lVar2 + 0x10) != (*(ulonglong *)(param_4 + 0x390) ^ 0x4a2178451bac3c7))) ||
       ((*(ulonglong *)(lVar2 + 0x18) != (*(ulonglong *)(param_4 + 0x398) ^ 0x4a1178751b9c3c6) ||
        (bVar1 = false,
        *(ulonglong *)(lVar2 + 0x20) != (*(ulonglong *)(param_4 + 0x3a0) ^ 0x4a0178651b8c3c5))))) {
      bVar1 = true;
    }
    if ((!bVar1) && (*(char *)(lVar2 + 2) == '\0')) {
      uVar3 = 0;
      goto LAB_ram_00007bb0;
    }
  }
  uVar3 = *(ulonglong *)(param_4 + 0x3a8) ^ 0xcf44133cb352d91c;
LAB_ram_00007bb0:
  param_1[1] = uVar3;
  *(undefined4 *)param_1 = 0;
  return;
}

// Function: FUN_ram_00007bd8
void FUN_ram_00007bd8(undefined4 *param_1,longlong *param_2,longlong param_3,ulonglong param_4,
                     longlong param_5)

{
  bool bVar1;
  undefined4 uVar2;
  longlong lVar3;
  longlong lVar4;
  ulonglong uVar5;
  ulonglong local_20;
  ulonglong local_18;
  ulonglong local_10;
  ulonglong local_8;
  
  uVar2 = 0;
  uVar5 = 0xbadc0de;
  if (param_3 == 2) {
    lVar4 = param_2[1];
    lVar3 = *param_2;
    local_8 = *(ulonglong *)(lVar4 + 0x270);
    local_10 = *(ulonglong *)(lVar4 + 0x268);
    local_18 = *(ulonglong *)(lVar4 + 0x260);
    local_20 = *(ulonglong *)(lVar4 + 600);
    if (*(ulonglong *)(lVar4 + 0x710) < 5) {
      local_18 = local_18 ^ 0x4a2178451bac3c7;
      local_20 = local_20 ^ 0xfb5ce87aae443c38;
      local_10 = local_10 ^ 0x4a1178751b9c3c6;
      local_8 = local_8 ^ 0x4a0178651b8c3c5;
    }
    if ((((local_20 != *(ulonglong *)(lVar3 + 8)) || (local_18 != *(ulonglong *)(lVar3 + 0x10))) ||
        (local_10 != *(ulonglong *)(lVar3 + 0x18))) ||
       (bVar1 = false, local_8 != *(ulonglong *)(lVar3 + 0x20))) {
      bVar1 = true;
    }
    uVar5 = 0xabad1dea;
    if ((*(char *)(lVar3 + 1) != '\0') && (!bVar1)) {
      if (param_5 != 0x90) {
                    /* WARNING: Subroutine does not return */
        FUN_ram_000011b0(&DAT_ram_000337df,10,2);
      }
      if ((param_4 & 7) != 0) {
                    /* WARNING: Subroutine does not return */
        FUN_ram_000011b0(&DAT_ram_000337df,10,0);
      }
      uVar5 = 0xbad1;
      if (((*(longlong *)(param_4 + 0x10) <= *(longlong *)(param_4 + 0x18)) &&
          (*(longlong *)(param_4 + 0x18) <= *(longlong *)(param_4 + 0x40))) &&
         ((*(longlong *)(param_4 + 0x40) <= *(longlong *)(param_4 + 0x48) &&
          ((*(longlong *)(param_4 + 0x48) <= *(longlong *)(param_4 + 0x70) &&
           (*(longlong *)(param_4 + 0x70) <= *(longlong *)(param_4 + 0x78))))))) {
        FUN_ram_00031b28(lVar4 + 0x58,param_4,0x90);
        uVar5 = *(ulonglong *)(lVar4 + 0x60) ^ 0x962f6f387c135a2c;
        *(ulonglong *)(lVar4 + 0x58) = *(ulonglong *)(lVar4 + 0x58) ^ 0x69d190c683eda5d3;
        *(ulonglong *)(lVar4 + 0x60) = uVar5;
        *(ulonglong *)(lVar4 + 0x68) = *(ulonglong *)(lVar4 + 0x68) ^ 0x962c6f3b7c105a2d;
        *(ulonglong *)(lVar4 + 0x70) = *(ulonglong *)(lVar4 + 0x70) ^ 0x962d6f3a7c115a2e;
        *(ulonglong *)(lVar4 + 0x78) = *(ulonglong *)(lVar4 + 0x78) ^ 0x962a6f3d7c165a2f;
        *(ulonglong *)(lVar4 + 0x80) = *(ulonglong *)(lVar4 + 0x80) ^ 0x962b6f3c7c175a28;
        *(ulonglong *)(lVar4 + 0x88) = *(ulonglong *)(lVar4 + 0x88) ^ 0x96286f3f7c145a29;
        *(ulonglong *)(lVar4 + 0x90) = *(ulonglong *)(lVar4 + 0x90) ^ 0x96296f3e7c155a2a;
        *(ulonglong *)(lVar4 + 0x98) = *(ulonglong *)(lVar4 + 0x98) ^ 0x96266f317c1a5a2b;
        *(ulonglong *)(lVar4 + 0xa0) = *(ulonglong *)(lVar4 + 0xa0) ^ 0x96276f307c1b5a24;
        *(ulonglong *)(lVar4 + 0xa8) = *(ulonglong *)(lVar4 + 0xa8) ^ 0x96246f337c185a25;
        *(ulonglong *)(lVar4 + 0xb0) = *(ulonglong *)(lVar4 + 0xb0) ^ 0x96256f327c195a26;
        *(ulonglong *)(lVar4 + 0xb8) = *(ulonglong *)(lVar4 + 0xb8) ^ 0x96226f357c1e5a27;
        *(ulonglong *)(lVar4 + 0xc0) = *(ulonglong *)(lVar4 + 0xc0) ^ 0x96236f347c1f5a20;
        *(ulonglong *)(lVar4 + 200) = *(ulonglong *)(lVar4 + 200) ^ 0x96206f377c1c5a21;
        *(ulonglong *)(lVar4 + 0xd0) = *(ulonglong *)(lVar4 + 0xd0) ^ 0x96216f367c1d5a22;
        *(ulonglong *)(lVar4 + 0xd8) = *(ulonglong *)(lVar4 + 0xd8) ^ 0x963e6f297c025a23;
        *(ulonglong *)(lVar4 + 0xe0) = *(ulonglong *)(lVar4 + 0xe0) ^ 0x963f6f287c035a3c;
        uVar2 = 0x1a;
      }
    }
  }
  param_1[1] = (int)uVar5;
  *param_1 = uVar2;
  return;
}

// Function: FUN_ram_00008188
void FUN_ram_00008188(undefined4 *param_1,longlong *param_2,longlong param_3,ulonglong param_4,
                     longlong param_5)

{
  bool bVar1;
  ulonglong uVar2;
  longlong lVar3;
  ulonglong uVar4;
  longlong lVar5;
  undefined4 *puVar6;
  ulonglong uVar7;
  ulonglong local_20;
  ulonglong local_18;
  ulonglong local_10;
  ulonglong local_8;
  
  uVar2 = 0;
  puVar6 = (undefined4 *)0xbadc0de;
  if (param_3 == 2) {
    lVar5 = param_2[1];
    lVar3 = *param_2;
    local_8 = *(ulonglong *)(lVar5 + 0x270);
    local_10 = *(ulonglong *)(lVar5 + 0x268);
    local_18 = *(ulonglong *)(lVar5 + 0x260);
    local_20 = *(ulonglong *)(lVar5 + 600);
    if (*(ulonglong *)(lVar5 + 0x710) < 5) {
      local_18 = local_18 ^ 0x4a2178451bac3c7;
      local_20 = local_20 ^ 0xfb5ce87aae443c38;
      local_10 = local_10 ^ 0x4a1178751b9c3c6;
      local_8 = local_8 ^ 0x4a0178651b8c3c5;
    }
    if ((((local_20 != *(ulonglong *)(lVar3 + 8)) || (local_18 != *(ulonglong *)(lVar3 + 0x10))) ||
        (local_10 != *(ulonglong *)(lVar3 + 0x18))) ||
       (bVar1 = false, local_8 != *(ulonglong *)(lVar3 + 0x20))) {
      bVar1 = true;
    }
    puVar6 = (undefined4 *)0xabad1dea;
    if ((*(char *)(lVar3 + 1) == '\0') || (bVar1)) goto LAB_ram_00008470;
    if (param_5 != 0xf0) {
                    /* WARNING: Subroutine does not return */
      FUN_ram_000011b0(&DAT_ram_000337df,10,2);
    }
    if ((param_4 & 7) != 0) {
                    /* WARNING: Subroutine does not return */
      FUN_ram_000011b0(&DAT_ram_000337df,10,0);
    }
    uVar4 = *(ulonglong *)(param_4 + 0x18);
    if (*(ulonglong *)(param_4 + 0x10) != 0 || uVar4 != 0) {
      if (0xffffffffffff < uVar4) goto LAB_ram_000085c8;
      puVar6 = (undefined4 *)0xbad2;
      if (*(ulonglong *)(param_4 + 0x30) < (*(ulonglong *)(param_4 + 0x10) >> 0x30 | uVar4 << 0x10))
      goto LAB_ram_00008470;
    }
    puVar6 = (undefined4 *)0xbad2;
    if (*(ulonglong *)(param_4 + 0x38) < *(ulonglong *)(param_4 + 0x30)) goto LAB_ram_00008470;
    uVar7 = *(ulonglong *)(param_4 + 0x68);
    uVar4 = *(ulonglong *)(param_4 + 0x80);
    if (*(ulonglong *)(param_4 + 0x60) != 0 || uVar7 != 0) {
      if (0xffffffffffff < uVar7) goto LAB_ram_000085c8;
      uVar2 = *(ulonglong *)(param_4 + 0x60) >> 0x30 | uVar7 << 0x10;
    }
    if (((uVar4 < *(ulonglong *)(param_4 + 0x38)) || (uVar4 < uVar2)) ||
       (*(ulonglong *)(param_4 + 0x88) < uVar4)) {
      uVar2 = 0;
    }
    else {
      uVar2 = *(ulonglong *)(param_4 + 0xb8);
      uVar4 = *(ulonglong *)(param_4 + 0xd0);
      uVar7 = 0;
      if (*(ulonglong *)(param_4 + 0xb0) != 0 || uVar2 != 0) {
        if (0xffffffffffff < uVar2) {
LAB_ram_000085c8:
                    /* WARNING: Subroutine does not return */
          FUN_ram_0002fb80(&DAT_ram_00034598);
        }
        uVar7 = *(ulonglong *)(param_4 + 0xb0) >> 0x30 | uVar2 << 0x10;
      }
      uVar2 = 0;
      if (((*(ulonglong *)(param_4 + 0x88) <= uVar4) && (uVar7 <= uVar4)) &&
         (uVar4 <= *(ulonglong *)(param_4 + 0xd8))) {
        FUN_ram_00031b28(lVar5 + 0xe8,param_4,0xf0);
        FUN_ram_00002a50(lVar5 + 0xe8);
        uVar2 = 0x1a;
        puVar6 = param_1;
      }
    }
  }
LAB_ram_00008470:
  param_1[1] = (int)puVar6;
  *param_1 = (int)uVar2;
  return;
}

// Function: FUN_ram_000085e0
void FUN_ram_000085e0(undefined8 *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                     undefined8 param_5)

{
  undefined *local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_21;
  undefined8 local_19;
  undefined8 local_11;
  undefined8 local_9;
  undefined1 local_1;
  
  local_68 = &DAT_ram_00033bb2;
  local_30 = 0x20;
  local_40 = 0x20;
  local_50 = 0x20;
  local_60 = 6;
  local_58 = param_2;
  local_48 = param_3;
  local_38 = param_4;
  FUN_ram_0002fa20(&local_21,&local_68,4,param_5);
  param_1[3] = local_9;
  param_1[2] = local_11;
  param_1[1] = local_19;
  *param_1 = local_21;
  *(undefined1 *)(param_1 + 4) = local_1;
  return;
}

// Function: FUN_ram_000086c8
void FUN_ram_000086c8(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 local_78;
  undefined8 local_70;
  undefined *local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_41;
  undefined8 local_39;
  undefined8 local_31;
  undefined8 local_29;
  undefined1 local_21;
  undefined8 local_20;
  undefined8 local_18;
  undefined8 local_10;
  undefined8 local_8;
  
  local_68 = &DAT_ram_00033580;
  local_50 = 0x20;
  local_60 = 0x20;
  local_70 = 0x20;
  local_8 = 0xd3bb8723dd54a054;
  local_10 = 0x6dd2523bce0a93a0;
  local_18 = 0x7a819dd33c7070c6;
  local_20 = 0xe959f7272b74fd7a;
  local_78 = param_2;
  local_58 = param_3;
  FUN_ram_0002fa20(&local_41,&local_78,3,&local_20);
  param_1[3] = local_29;
  param_1[2] = local_31;
  param_1[1] = local_39;
  *param_1 = local_41;
  *(undefined1 *)(param_1 + 4) = local_21;
  return;
}

// Function: FUN_ram_00008808
void FUN_ram_00008808(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 local_78;
  undefined8 local_70;
  undefined *local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_41;
  undefined8 local_39;
  undefined8 local_31;
  undefined8 local_29;
  undefined1 local_21;
  undefined8 local_20;
  undefined8 local_18;
  undefined8 local_10;
  undefined8 local_8;
  
  local_68 = &DAT_ram_00033600;
  local_50 = 0x20;
  local_60 = 0x20;
  local_70 = 0x20;
  local_8 = 0xd3bb8723dd54a054;
  local_10 = 0x6dd2523bce0a93a0;
  local_18 = 0x7a819dd33c7070c6;
  local_20 = 0xe959f7272b74fd7a;
  local_78 = param_2;
  local_58 = param_3;
  FUN_ram_0002fa20(&local_41,&local_78,3,&local_20);
  param_1[3] = local_29;
  param_1[2] = local_31;
  param_1[1] = local_39;
  *param_1 = local_41;
  *(undefined1 *)(param_1 + 4) = local_21;
  return;
}

// Function: FUN_ram_00008948
void FUN_ram_00008948(int *param_1,undefined8 *param_2,char *param_3,longlong *param_4,
                     longlong param_5)

{
  char cVar1;
  char *pcVar2;
  int iVar3;
  char *pcVar4;
  byte *pbVar5;
  longlong lVar6;
  undefined1 uVar7;
  longlong *plVar8;
  int iStack_150;
  int iStack_14c;
  undefined1 local_141;
  longlong local_140 [2];
  undefined *local_130;
  undefined8 local_128;
  longlong local_120;
  undefined8 local_118;
  undefined1 *local_110;
  undefined8 local_108;
  longlong *local_100;
  undefined8 local_f8;
  char *local_f0;
  undefined2 local_e8;
  byte *local_e0;
  undefined2 local_d8;
  undefined4 local_cc;
  undefined8 local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined8 local_b0;
  undefined8 local_a8;
  undefined8 local_a0;
  char *local_98;
  longlong *local_90;
  longlong local_88;
  char *local_80;
  char *local_78;
  undefined8 local_70;
  undefined1 local_68;
  undefined1 local_67;
  undefined1 local_66;
  byte *local_60;
  byte *local_58;
  undefined8 local_50;
  byte *local_48;
  byte *local_40;
  undefined8 local_38;
  undefined1 local_30;
  undefined1 local_2f;
  undefined1 local_2e;
  undefined *local_28;
  char **local_20;
  undefined8 local_18;
  undefined4 *local_10;
  undefined8 local_8;
  
  local_141 = (undefined1)*(undefined8 *)(param_5 + -0xff8);
  lVar6 = *param_4;
  plVar8 = *(longlong **)(param_5 + -0x1000);
  local_110 = &local_141;
  local_130 = &DAT_ram_00033580;
  local_120 = *plVar8 + 8;
  local_108 = 1;
  local_118 = 0x20;
  local_128 = 0x20;
  local_140[1] = 0x20;
  local_100 = local_140;
  local_f8 = 4;
  pcVar2 = (char *)*param_2;
  pbVar5 = *(byte **)param_3;
  local_e0 = pbVar5 + 8;
  local_f0 = pcVar2 + 8;
  local_d8 = 0x101;
  local_e8 = 0x101;
  local_b8 = 0x93a165d7e1f6dd06;
  local_b0 = 0xac79ebce46e1cbd9;
  local_a8 = 0x91375b5fed85b41c;
  local_a0 = 0xa900ff7e85f58c3a;
  local_c0 = 0xa5;
  local_c8 = 0x1f1df0;
  local_cc = 0;
  pcVar4 = param_3;
  if (*pcVar2 == -1) {
    local_68 = 1;
    if (pcVar2[1] == '\0') {
      local_68 = 0;
      if (pcVar2[2] == '\0') goto LAB_ram_00008c18;
LAB_ram_00008b20:
      uVar7 = 1;
      local_67 = 1;
      cVar1 = pcVar2[3];
    }
    else {
      if (pcVar2[2] != '\0') goto LAB_ram_00008b20;
LAB_ram_00008c18:
      uVar7 = 0;
      local_67 = 0;
      cVar1 = pcVar2[3];
    }
    if (cVar1 == '\0') {
      local_67 = uVar7;
    }
    local_66 = cVar1 != '\0';
    local_88 = *(longlong *)(pcVar2 + 0x50);
    local_78 = pcVar2 + 0x28;
    local_80 = pcVar2 + 0x58;
    local_90 = (longlong *)(pcVar2 + 0x48);
    local_70 = 0;
    pcVar4 = (char *)(ulonglong)*pbVar5;
    if (pcVar4 == (char *)0xff) {
      local_50 = *(undefined8 *)(pbVar5 + 0x50);
      local_40 = pbVar5 + 0x28;
      local_48 = pbVar5 + 0x58;
      local_58 = pbVar5 + 0x48;
      if (pbVar5[3] == 0) {
        local_2e = 0;
        if (pbVar5[2] == 0) goto LAB_ram_00008e48;
LAB_ram_00008cd8:
        local_2f = 1;
      }
      else {
        local_2e = 1;
        if (pbVar5[2] != 0) goto LAB_ram_00008cd8;
LAB_ram_00008e48:
        local_2f = 0;
      }
      local_30 = pbVar5[1] != 0;
      local_38 = 0;
      local_10 = &local_cc;
      local_20 = &local_f0;
      local_28 = &DAT_ram_000335a0;
      local_8 = 0x34;
      local_18 = 2;
      local_140[0] = lVar6 + 8;
      local_98 = local_f0;
      local_60 = local_e0;
      FUN_ram_00008da8(&local_28,&local_98,2,&local_100,1);
      iVar3 = 8;
      local_98 = param_3;
      local_90 = plVar8;
      local_88 = lVar6 + 8;
      FUN_ram_0002d300(&iStack_150,&local_98,8,0);
      if (iStack_150 == 0x1a) {
        iStack_150 = 0x1a;
        iStack_14c = iVar3;
      }
      goto LAB_ram_00008be0;
    }
  }
  iStack_150 = 0xb;
  iStack_14c = (int)pcVar4;
LAB_ram_00008be0:
  param_1[1] = iStack_14c;
  *param_1 = iStack_150;
  return;
}

// Function: FUN_ram_00008da8
void FUN_ram_00008da8(void)

{
  int iVar1;
  int *unaff_R8;
  int local_150;
  int local_14c;
  
  FUN_ram_00008da8();
  iVar1 = 8;
  FUN_ram_0002d300(&local_150,&stack0xffffffffffffff68,8,0);
  if (local_150 == 0x1a) {
    local_150 = 0x1a;
    local_14c = iVar1;
  }
  unaff_R8[1] = local_14c;
  *unaff_R8 = local_150;
  return;
}

// Function: FUN_ram_00008e90
void FUN_ram_00008e90(undefined4 *param_1,undefined8 *param_2,undefined8 *param_3,
                     undefined8 *param_4,longlong param_5)

{
  char cVar1;
  byte bVar2;
  undefined1 uVar3;
  byte *pbVar4;
  undefined8 uVar5;
  undefined4 uVar6;
  byte *pbVar7;
  char *pcVar8;
  byte *pbVar9;
  char *pcVar10;
  byte *pbVar11;
  undefined1 local_181;
  byte *local_180;
  undefined8 local_178;
  undefined *local_170;
  undefined8 local_168;
  byte *local_160;
  undefined8 local_158;
  undefined1 *local_150;
  undefined8 local_148;
  byte **local_140;
  undefined8 local_138;
  undefined1 local_130;
  undefined7 uStack_12f;
  undefined1 local_128;
  undefined1 uStack_127;
  undefined6 uStack_126;
  undefined1 local_120;
  undefined7 uStack_11f;
  undefined1 local_118;
  undefined1 uStack_117;
  undefined7 uStack_116;
  undefined8 local_108;
  undefined2 uStack_100;
  undefined2 uStack_fe;
  undefined4 local_fc;
  undefined8 uStack_f8;
  undefined2 uStack_f0;
  undefined2 uStack_ee;
  undefined4 local_ec;
  undefined8 uStack_e8;
  undefined2 uStack_e0;
  undefined2 uStack_de;
  undefined8 local_dc;
  char *local_d0;
  char *local_c8;
  undefined8 local_c0;
  char *local_b8;
  char *local_b0;
  undefined8 local_a8;
  undefined1 local_a0;
  undefined1 local_9f;
  undefined1 local_9e;
  byte *local_98;
  byte *local_90;
  undefined8 local_88;
  byte *local_80;
  byte *local_78;
  undefined8 local_70;
  undefined1 local_68;
  undefined1 local_67;
  undefined1 local_66;
  byte *pbStack_60;
  byte *pbStack_58;
  undefined8 uStack_50;
  byte *pbStack_48;
  byte *pbStack_40;
  undefined8 uStack_38;
  undefined1 uStack_30;
  undefined1 uStack_2f;
  undefined1 uStack_2e;
  undefined *local_28;
  undefined8 *local_20;
  undefined8 local_18;
  undefined8 *local_10;
  undefined8 local_8;
  
  local_181 = (undefined1)*(undefined8 *)(param_5 + -0xff8);
  pbVar4 = (byte *)*param_4;
  pbVar7 = (byte *)**(undefined8 **)(param_5 + -0x1000);
  local_150 = &local_181;
  local_170 = &DAT_ram_00033600;
  pbVar11 = pbVar7 + 8;
  pbVar9 = pbVar4 + 8;
  local_148 = 1;
  local_158 = 0x20;
  local_168 = 0x20;
  local_178 = 0x20;
  local_140 = &local_180;
  local_138 = 4;
  local_c8 = (char *)*param_2;
  pcVar10 = (char *)*param_3;
  pcVar8 = pcVar10 + 8;
  local_120 = SUB81(pcVar8,0);
  uStack_11f = (undefined7)((ulonglong)pcVar8 >> 8);
  local_d0 = local_c8 + 8;
  local_130 = SUB81(local_d0,0);
  uStack_12f = (undefined7)((ulonglong)local_d0 >> 8);
  local_118 = 1;
  uStack_117 = 1;
  local_128 = 1;
  uStack_127 = 1;
  uStack_f8._4_4_ = 0xe1f6dd06;
  uStack_f0 = 0x75ee;
  uStack_ee = 0xde8f;
  local_ec = 0xbc5d4218;
  uStack_e8._0_4_ = 0xdacd6ce4;
  uStack_e8._4_4_ = 0x4dfc1ab6;
  uStack_e0 = 0xb983;
  uStack_de = 0x270d;
  local_dc = 0xfc8ba1d828f9bdfe;
  local_fc = 200;
  uStack_f8._0_4_ = 0;
  local_108._4_4_ = 0x22d580;
  uStack_100 = 0;
  uStack_fe = 0;
  local_108._0_4_ = 0;
  if (*local_c8 == -1) {
    local_a0 = 1;
    if (local_c8[1] == '\0') {
      local_a0 = 0;
      if (local_c8[2] != '\0') goto LAB_ram_00009090;
LAB_ram_00009178:
      uVar3 = 0;
      local_9f = 0;
      cVar1 = local_c8[3];
    }
    else {
      if (local_c8[2] == '\0') goto LAB_ram_00009178;
LAB_ram_00009090:
      uVar3 = 1;
      local_9f = 1;
      cVar1 = local_c8[3];
    }
    if (cVar1 == '\0') {
      local_9f = uVar3;
    }
    local_9e = cVar1 != '\0';
    local_c0 = *(undefined8 *)(local_c8 + 0x50);
    local_b0 = local_c8 + 0x28;
    local_b8 = local_c8 + 0x58;
    local_c8 = local_c8 + 0x48;
    local_a8 = 0;
    if (*pcVar10 == -1) {
      local_68 = 1;
      if (pcVar10[1] == '\0') {
        local_68 = 0;
        if (pcVar10[2] != '\0') goto LAB_ram_000091d8;
LAB_ram_00009738:
        uVar3 = 0;
        local_67 = 0;
        cVar1 = pcVar10[3];
      }
      else {
        if (pcVar10[2] == '\0') goto LAB_ram_00009738;
LAB_ram_000091d8:
        uVar3 = 1;
        local_67 = 1;
        cVar1 = pcVar10[3];
      }
      if (cVar1 == '\0') {
        local_67 = uVar3;
      }
      local_66 = cVar1 != '\0';
      local_88 = *(undefined8 *)(pcVar10 + 0x50);
      local_70 = 0;
      local_10 = &local_108;
      local_20 = (undefined8 *)&local_130;
      local_28 = &DAT_ram_000335a0;
      local_8 = 0x34;
      local_18 = 2;
      local_180 = pbVar9;
      local_160 = pbVar11;
      local_98 = (byte *)pcVar8;
      local_90 = (byte *)(pcVar10 + 0x48);
      local_80 = (byte *)(pcVar10 + 0x58);
      local_78 = (byte *)(pcVar10 + 0x28);
      FUN_ram_00009328(&local_28,&local_d0,2,&local_140,1);
      local_130 = 0x12;
      uStack_12f = (undefined7)*(undefined8 *)pbVar9;
      local_128 = (undefined1)((ulonglong)*(undefined8 *)pbVar9 >> 0x38);
      uVar5 = *(undefined8 *)(pbVar4 + 0x10);
      uStack_127 = (undefined1)uVar5;
      uStack_126 = (undefined6)((ulonglong)uVar5 >> 8);
      local_120 = (undefined1)((ulonglong)uVar5 >> 0x38);
      uStack_11f = (undefined7)*(undefined8 *)(pbVar4 + 0x18);
      local_118 = (undefined1)((ulonglong)*(undefined8 *)(pbVar4 + 0x18) >> 0x38);
      uStack_117 = (undefined1)*(undefined8 *)(pbVar4 + 0x20);
      uStack_116 = (undefined7)((ulonglong)*(undefined8 *)(pbVar4 + 0x20) >> 8);
      uStack_e0 = 0;
      uStack_f0 = 0;
      uStack_100 = 1;
      if (*pcVar10 == -1) {
        local_a0 = 1;
        if (pcVar10[1] == '\0') {
          local_a0 = 0;
          if (pcVar10[2] != '\0') goto LAB_ram_00009408;
LAB_ram_00009788:
          uVar3 = 0;
          local_9f = 0;
          cVar1 = pcVar10[3];
        }
        else {
          if (pcVar10[2] == '\0') goto LAB_ram_00009788;
LAB_ram_00009408:
          uVar3 = 1;
          local_9f = 1;
          cVar1 = pcVar10[3];
        }
        if (cVar1 == '\0') {
          local_9f = uVar3;
        }
        local_9e = cVar1 != '\0';
        local_c0 = *(undefined8 *)(pcVar10 + 0x50);
        local_a8 = 0;
        if ((*pbVar7 & 0x88) == 0x88) {
          local_68 = 1;
          if (pbVar7[1] == 0) {
            local_68 = 0;
            if (pbVar7[2] != 0) goto LAB_ram_000094f8;
LAB_ram_000097c8:
            uVar3 = 0;
            local_67 = 0;
            bVar2 = pbVar7[3];
          }
          else {
            if (pbVar7[2] == 0) goto LAB_ram_000097c8;
LAB_ram_000094f8:
            uVar3 = 1;
            local_67 = 1;
            bVar2 = pbVar7[3];
          }
          if (bVar2 == 0) {
            local_67 = uVar3;
          }
          local_66 = bVar2 != 0;
          local_88 = *(undefined8 *)(pbVar7 + 0x50);
          local_78 = pbVar7 + 0x28;
          local_80 = pbVar7 + 0x58;
          local_90 = pbVar7 + 0x48;
          local_70 = 0;
          if ((*pbVar4 & 0x88) == 0x88) {
            uStack_50 = *(undefined8 *)(pbVar4 + 0x50);
            pbStack_40 = pbVar4 + 0x28;
            pbStack_48 = pbVar4 + 0x58;
            pbStack_58 = pbVar4 + 0x48;
            if (pbVar4[3] == 0) {
              uStack_2e = 0;
              if (pbVar4[2] != 0) goto LAB_ram_00009650;
LAB_ram_00009800:
              uStack_2f = 0;
            }
            else {
              uStack_2e = 1;
              if (pbVar4[2] == 0) goto LAB_ram_00009800;
LAB_ram_00009650:
              uStack_2f = 1;
            }
            uStack_30 = pbVar4[1] != 0;
            uStack_38 = 0;
            local_10 = (undefined8 *)&local_130;
            local_20 = &local_108;
            local_28 = &DAT_ram_00033600;
            local_8 = 0x21;
            local_18 = 3;
            local_d0 = pcVar8;
            local_c8 = pcVar10 + 0x48;
            local_b8 = pcVar10 + 0x58;
            local_b0 = pcVar10 + 0x28;
            local_98 = pbVar11;
            pbStack_60 = pbVar9;
            uStack_e8 = pbVar9;
            uStack_f8 = pbVar11;
            local_108 = pcVar8;
            FUN_ram_00009708(&local_28,&local_d0,3,8,0);
            uVar6 = 0x1a;
            goto LAB_ram_00009150;
          }
        }
      }
      uVar6 = 0xb;
      goto LAB_ram_00009150;
    }
  }
  uVar6 = 0xb;
LAB_ram_00009150:
  *param_1 = uVar6;
  return;
}

// Function: FUN_ram_00009328
void FUN_ram_00009328(void)

{
  undefined4 uVar1;
  undefined8 *unaff_R7;
  char *unaff_R8;
  byte *local_1a8;
  byte *local_1a0;
  undefined4 *local_190;
  undefined1 local_130;
  undefined8 local_12f;
  undefined8 local_127;
  undefined8 local_11f;
  undefined8 local_117;
  undefined *local_28;
  undefined1 *local_20;
  undefined8 local_18;
  undefined1 *local_10;
  undefined8 local_8;
  
  FUN_ram_00009328();
  local_130 = 0x12;
  local_12f = *unaff_R7;
  local_127 = unaff_R7[1];
  local_11f = unaff_R7[2];
  local_117 = unaff_R7[3];
  if (((*unaff_R8 == -1) && ((*local_1a8 & 0x88) == 0x88)) && ((*local_1a0 & 0x88) == 0x88)) {
    local_10 = &local_130;
    local_20 = &stack0xfffffffffffffef8;
    local_28 = &DAT_ram_00033600;
    local_8 = 0x21;
    local_18 = 3;
    FUN_ram_00009708(&local_28,&stack0xffffffffffffff30,3,8,0);
    uVar1 = 0x1a;
  }
  else {
    uVar1 = 0xb;
  }
  *local_190 = uVar1;
  return;
}

// Function: FUN_ram_00009708
void FUN_ram_00009708(void)

{
  undefined8 uStack_190;
  
  FUN_ram_00009708();
  *uStack_190 = 0x1a;
  return;
}

// Function: FUN_ram_00009820
void FUN_ram_00009820(int *param_1,undefined8 *param_2,longlong param_3,ulonglong param_4,
                     longlong param_5)

{
  char cVar1;
  bool bVar2;
  int iVar3;
  longlong *plVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined1 uVar7;
  undefined8 *puVar8;
  undefined1 uVar9;
  undefined1 uVar10;
  char *pcVar11;
  char *pcVar12;
  int iVar13;
  undefined8 *puVar14;
  char *pcVar15;
  longlong *plVar16;
  int iStack_1c8;
  uint uStack_1c4;
  int iStack_1c0;
  uint uStack_1bc;
  undefined *local_1b8;
  longlong local_1b0;
  longlong local_1a8;
  longlong local_1a0;
  char local_191;
  undefined *local_190;
  longlong local_188;
  undefined8 *local_180;
  longlong local_178;
  longlong *local_170;
  undefined8 local_168;
  longlong *local_160;
  undefined8 local_158;
  char *local_150;
  undefined8 local_148;
  undefined **local_140;
  undefined8 local_138;
  longlong lStack_130;
  longlong lStack_128;
  longlong lStack_120;
  longlong lStack_118;
  longlong lStack_110;
  longlong lStack_108;
  longlong lStack_100;
  longlong lStack_f8;
  undefined **local_f0;
  undefined2 local_e8;
  char *local_e0;
  undefined2 local_d8;
  undefined4 local_cc;
  undefined4 local_c8;
  undefined4 uStack_c4;
  undefined4 local_c0;
  undefined4 uStack_bc;
  undefined4 local_b8;
  undefined4 uStack_b4;
  undefined4 local_b0;
  undefined4 uStack_ac;
  undefined8 local_a8;
  undefined8 local_a0;
  undefined **local_98;
  char *local_90;
  undefined8 *local_88;
  char *local_80;
  longlong *local_78;
  undefined8 local_70;
  longlong *local_68;
  char *local_60;
  char *local_58;
  undefined8 local_50;
  char *local_48;
  char *local_40;
  undefined8 local_38;
  undefined1 local_30;
  undefined1 local_2f;
  undefined1 local_2e;
  undefined *local_28;
  longlong **local_20;
  undefined8 local_18;
  undefined4 *local_10;
  undefined8 local_8;
  
  iVar3 = 0;
  iVar13 = 0xbadc0de;
  if (param_3 != 8) goto LAB_ram_00009d90;
  iVar13 = -0x5452e216;
  pcVar11 = (char *)*param_2;
  if (pcVar11[1] == '\0') goto LAB_ram_00009d90;
  if ((((*(longlong *)(pcVar11 + 8) != -0x32b20de6c4775e40) ||
       (*(longlong *)(pcVar11 + 0x10) != -0x15200f0917dbfd5f)) ||
      (*(longlong *)(pcVar11 + 0x18) != -0x47ebde6f8ba50bd0)) ||
     (bVar2 = false, *(longlong *)(pcVar11 + 0x20) != 0x4164f987ea077494)) {
    bVar2 = true;
  }
  if (bVar2) goto LAB_ram_00009d90;
  if (param_5 != 0x200) {
                    /* WARNING: Subroutine does not return */
    FUN_ram_000011b0(&DAT_ram_000337df,10,2);
  }
  if ((param_4 & 7) != 0) {
                    /* WARNING: Subroutine does not return */
    FUN_ram_000011b0(&DAT_ram_000337df,10,0);
  }
  puVar14 = param_2 + 1;
  local_98 = (undefined **)&DAT_ram_00033bb2;
  plVar4 = (longlong *)(param_4 + 0x180);
  plVar16 = (longlong *)(param_4 + 0x1a0);
  puVar8 = (undefined8 *)(param_4 + 0x1c0);
  local_60 = (char *)0x20;
  local_70 = 0x20;
  local_80 = (char *)0x20;
  local_90 = (char *)0x6;
  local_88 = puVar8;
  local_78 = plVar16;
  local_68 = plVar4;
  FUN_ram_0002fa20(&local_190,&local_98,4,&DAT_ram_00033500);
  local_1a0 = local_178;
  local_1a8 = (longlong)local_180;
  local_1b0 = local_188;
  local_1b8 = local_190;
  local_191 = (char)local_170;
  pcVar15 = (char *)*puVar14;
  local_e0 = pcVar15 + 8;
  if (((local_190 != *(undefined **)(pcVar15 + 8)) || (local_188 != *(longlong *)(pcVar15 + 0x10)))
     || ((local_180 != (undefined8 *)*(longlong *)(pcVar15 + 0x18) ||
         (bVar2 = false, local_178 != *(longlong *)(pcVar15 + 0x20))))) {
    bVar2 = true;
  }
  if (bVar2) {
    iVar3 = 0;
    iVar13 = 0xbadface;
    goto LAB_ram_00009d90;
  }
  local_150 = &local_191;
  local_190 = &DAT_ram_00033bb2;
  local_148 = 1;
  local_158 = 0x20;
  local_168 = 0x20;
  local_178 = 0x20;
  local_188 = 6;
  local_140 = &local_190;
  local_138 = 5;
  local_d8 = 0x101;
  local_e8 = 0x101;
  local_b8 = 0x2b74fd7a;
  uStack_b4 = 0xe959f727;
  local_b0 = 0x3c7070c6;
  uStack_ac = 0x7a819dd3;
  local_a8 = 0x6dd2523bce0a93a0;
  local_a0 = 0xd3bb8723dd54a054;
  local_c0 = 0x6c0;
  uStack_bc = 0;
  local_c8 = 0xc51c00;
  uStack_c4 = 0;
  local_cc = 0;
  local_80 = pcVar15;
  if (*pcVar11 == -1) {
    uVar7 = 1;
    if (pcVar11[1] == '\0') {
      uVar7 = 0;
      if (pcVar11[2] == '\0') goto LAB_ram_00009dc8;
LAB_ram_00009cd0:
      uVar10 = 1;
      uVar9 = 1;
      cVar1 = pcVar11[3];
    }
    else {
      if (pcVar11[2] != '\0') goto LAB_ram_00009cd0;
LAB_ram_00009dc8:
      uVar10 = 0;
      uVar9 = 0;
      cVar1 = pcVar11[3];
    }
    if (cVar1 == '\0') {
      uVar9 = uVar10;
    }
    local_88 = *(undefined8 **)(pcVar11 + 0x50);
    local_78 = (longlong *)(pcVar11 + 0x28);
    local_80 = pcVar11 + 0x58;
    local_90 = pcVar11 + 0x48;
    local_68 = (longlong *)CONCAT71(CONCAT61(CONCAT51(local_68._3_5_,cVar1 != '\0'),uVar9),uVar7);
    local_70 = 0;
    if (*pcVar15 == -1) {
      local_50 = *(undefined8 *)(pcVar15 + 0x50);
      local_40 = pcVar15 + 0x28;
      local_58 = pcVar15 + 0x48;
      local_2e = pcVar15[3] != '\0';
      local_2f = pcVar15[2] != '\0';
      local_30 = pcVar15[1] != '\0';
      pcVar12 = pcVar15 + 0x58;
      local_38 = 0;
      local_10 = &local_cc;
      local_20 = (longlong **)&local_f0;
      local_28 = &DAT_ram_000335a0;
      local_8 = 0x34;
      local_18 = 2;
      local_180 = puVar8;
      local_170 = plVar16;
      local_160 = plVar4;
      local_f0 = (undefined **)(pcVar11 + 8);
      local_98 = (undefined **)(pcVar11 + 8);
      local_60 = local_e0;
      local_48 = pcVar12;
      FUN_ram_00009f80(&local_28,&local_98,2,&local_140,1);
      local_88 = (undefined8 *)&DAT_ram_00033580;
      local_70 = 0x20;
      local_80 = (char *)0x20;
      local_90 = (char *)0x20;
      local_10 = (undefined4 *)0xd3bb8723dd54a054;
      local_18 = 0x6dd2523bce0a93a0;
      local_20 = (longlong **)0x7a819dd33c7070c6;
      local_28 = (undefined *)0xe959f7272b74fd7a;
      local_98 = &local_1b8;
      local_78 = plVar16;
      FUN_ram_0002fa20(&local_cc,&local_98,3,&local_28);
      lStack_118 = CONCAT44(local_b0,uStack_b4);
      lStack_120 = CONCAT44(local_b8,uStack_bc);
      lStack_128 = CONCAT44(local_c0,uStack_c4);
      lStack_130 = CONCAT44(local_c8,local_cc);
      local_88 = (undefined8 *)&DAT_ram_00033580;
      local_70 = 0x20;
      local_80 = (char *)0x20;
      local_90 = (char *)0x20;
      local_10 = (undefined4 *)0xd3bb8723dd54a054;
      local_18 = 0x6dd2523bce0a93a0;
      local_20 = (longlong **)0x7a819dd33c7070c6;
      local_28 = (undefined *)0xe959f7272b74fd7a;
      local_98 = &local_1b8;
      local_78 = plVar4;
      FUN_ram_0002fa20(&local_cc,&local_98,3,&local_28);
      lStack_f8 = CONCAT44(local_b0,uStack_b4);
      lStack_100 = CONCAT44(local_b8,uStack_bc);
      lStack_108 = CONCAT44(local_c0,uStack_c4);
      lStack_110 = CONCAT44(local_c8,local_cc);
      lVar5 = param_2[4];
      if (((*plVar16 != *(longlong *)(lVar5 + 8)) ||
          (*(longlong *)(param_4 + 0x1a8) != *(longlong *)(lVar5 + 0x10))) ||
         ((*(longlong *)(param_4 + 0x1b0) != *(longlong *)(lVar5 + 0x18) ||
          (bVar2 = false, *(longlong *)(param_4 + 0x1b8) != *(longlong *)(lVar5 + 0x20))))) {
        bVar2 = true;
      }
      iVar3 = 0;
      iVar13 = -0x5452e216;
      if (!bVar2) {
        lVar5 = param_2[5];
        if ((((*plVar4 != *(longlong *)(lVar5 + 8)) ||
             (*(longlong *)(param_4 + 0x188) != *(longlong *)(lVar5 + 0x10))) ||
            (*(longlong *)(param_4 + 400) != *(longlong *)(lVar5 + 0x18))) ||
           (bVar2 = false, *(longlong *)(param_4 + 0x198) != *(longlong *)(lVar5 + 0x20))) {
          bVar2 = true;
        }
        if (!bVar2) {
          lVar5 = param_2[2];
          if (((lStack_130 != *(longlong *)(lVar5 + 8)) ||
              (lStack_128 != *(longlong *)(lVar5 + 0x10))) ||
             ((lStack_120 != *(longlong *)(lVar5 + 0x18) ||
              (bVar2 = false, lStack_118 != *(longlong *)(lVar5 + 0x20))))) {
            bVar2 = true;
          }
          iVar13 = -0x4520531d;
          if (!bVar2) {
            lVar5 = param_2[3];
            if (((lStack_110 != *(longlong *)(lVar5 + 8)) ||
                (lStack_108 != *(longlong *)(lVar5 + 0x10))) ||
               ((lStack_100 != *(longlong *)(lVar5 + 0x18) ||
                (bVar2 = false, lStack_f8 != *(longlong *)(lVar5 + 0x20))))) {
              bVar2 = true;
            }
            if (!bVar2) {
              FUN_ram_00008948(&iStack_1c0,param_2,param_2 + 2,puVar14);
              if (iStack_1c0 == 0x1a) {
                FUN_ram_00008948(&iStack_1c8,param_2,param_2 + 3,puVar14);
                if (iStack_1c8 == 0x1a) {
                  FUN_ram_00016c40(pcVar12,*(undefined8 *)(pcVar15 + 0x50));
                  FUN_ram_00031b28(pcVar12,param_4 + 0xf0,0x90);
                  FUN_ram_00031b28(pcVar15 + 0xe8,param_4,0xf0);
                  *(undefined8 *)(pcVar15 + 0x270) = *(undefined8 *)(param_4 + 0x1d8);
                  *(undefined8 *)(pcVar15 + 0x268) = *(undefined8 *)(param_4 + 0x1d0);
                  *(undefined8 *)(pcVar15 + 0x260) = *(undefined8 *)(param_4 + 0x1c8);
                  *(undefined8 *)(pcVar15 + 600) = *puVar8;
                  *(undefined8 *)(pcVar15 + 0x210) = *(undefined8 *)(param_4 + 0x1b8);
                  *(undefined8 *)(pcVar15 + 0x208) = *(undefined8 *)(param_4 + 0x1b0);
                  *(undefined8 *)(pcVar15 + 0x200) = *(undefined8 *)(param_4 + 0x1a8);
                  *(longlong *)(pcVar15 + 0x1f8) = *plVar16;
                  *(longlong *)(pcVar15 + 0x1d8) = *plVar4;
                  *(undefined8 *)(pcVar15 + 0x1e0) = *(undefined8 *)(param_4 + 0x188);
                  *(undefined8 *)(pcVar15 + 0x1e8) = *(undefined8 *)(param_4 + 400);
                  *(undefined8 *)(pcVar15 + 0x1f0) = *(undefined8 *)(param_4 + 0x198);
                  *(longlong *)(pcVar15 + 0x238) = lStack_130;
                  *(longlong *)(pcVar15 + 0x240) = lStack_128;
                  *(longlong *)(pcVar15 + 0x248) = lStack_120;
                  *(longlong *)(pcVar15 + 0x250) = lStack_118;
                  *(longlong *)(pcVar15 + 0x218) = lStack_110;
                  *(longlong *)(pcVar15 + 0x220) = lStack_108;
                  *(longlong *)(pcVar15 + 0x228) = lStack_100;
                  *(longlong *)(pcVar15 + 0x230) = lStack_f8;
                  pcVar15[0x290] = local_191;
                  pcVar15[0x291] = '\0';
                  pcVar15[0x292] = '\0';
                  pcVar15[0x293] = '\0';
                  pcVar15[0x294] = '\0';
                  pcVar15[0x294] = '\0';
                  pcVar15[0x295] = '\0';
                  pcVar15[0x296] = '\0';
                  pcVar15[0x297] = '\0';
                  pcVar15[0x288] = *(char *)(param_4 + 0x1f0);
                  pcVar15[0x289] = '\0';
                  pcVar15[0x28a] = '\0';
                  pcVar15[0x28b] = '\0';
                  pcVar15[0x28c] = '\0';
                  pcVar15[0x28c] = '\0';
                  pcVar15[0x28d] = '\0';
                  pcVar15[0x28e] = '\0';
                  pcVar15[0x28f] = '\0';
                  uVar6 = *(undefined8 *)(param_4 + 0x1e0);
                  *(undefined8 *)(pcVar15 + 0x280) = *(undefined8 *)(param_4 + 0x1e8);
                  *(undefined8 *)(pcVar15 + 0x278) = uVar6;
                  pcVar15[0x328] = '\0';
                  pcVar15[0x329] = '\0';
                  pcVar15[0x32a] = '\0';
                  pcVar15[0x32b] = '\0';
                  pcVar15[0x32c] = '\0';
                  pcVar15[0x32d] = '\0';
                  pcVar15[0x32e] = '\0';
                  pcVar15[0x32f] = '\0';
                  pcVar15[0x330] = '\0';
                  pcVar15[0x331] = '\0';
                  pcVar15[0x332] = '\0';
                  pcVar15[0x333] = '\0';
                  pcVar15[0x334] = '\0';
                  pcVar15[0x335] = '\0';
                  pcVar15[0x336] = '\0';
                  pcVar15[0x337] = '\0';
                  pcVar15[0x338] = '\0';
                  pcVar15[0x339] = '\0';
                  pcVar15[0x33a] = '\0';
                  pcVar15[0x33b] = '\0';
                  pcVar15[0x33c] = '\0';
                  pcVar15[0x33d] = '\0';
                  pcVar15[0x33e] = '\0';
                  pcVar15[0x33f] = '\0';
                  pcVar15[0x340] = '\0';
                  pcVar15[0x341] = '\0';
                  pcVar15[0x342] = '\0';
                  pcVar15[0x343] = '\0';
                  pcVar15[0x344] = '\0';
                  pcVar15[0x345] = '\0';
                  pcVar15[0x346] = '\0';
                  pcVar15[0x347] = '\0';
                  pcVar15[0x348] = '\0';
                  pcVar15[0x349] = '\0';
                  pcVar15[0x34a] = '\0';
                  pcVar15[0x34b] = '\0';
                  pcVar15[0x34c] = '\0';
                  pcVar15[0x34d] = '\0';
                  pcVar15[0x34e] = '\0';
                  pcVar15[0x34f] = '\0';
                  pcVar15[0x350] = '\0';
                  pcVar15[0x351] = '\0';
                  pcVar15[0x352] = '\0';
                  pcVar15[0x353] = '\0';
                  pcVar15[0x354] = '\0';
                  pcVar15[0x355] = '\0';
                  pcVar15[0x356] = '\0';
                  pcVar15[0x357] = '\0';
                  pcVar15[0x358] = '\0';
                  pcVar15[0x359] = '\0';
                  pcVar15[0x35a] = '\0';
                  pcVar15[0x35b] = '\0';
                  pcVar15[0x35c] = '\0';
                  pcVar15[0x35d] = '\0';
                  pcVar15[0x35e] = '\0';
                  pcVar15[0x35f] = '\0';
                  pcVar15[0x360] = '\0';
                  pcVar15[0x361] = '\0';
                  pcVar15[0x362] = '\0';
                  pcVar15[0x363] = '\0';
                  pcVar15[0x364] = '\0';
                  pcVar15[0x365] = '\0';
                  pcVar15[0x366] = '\0';
                  pcVar15[0x367] = '\0';
                  pcVar15[0x368] = '\0';
                  pcVar15[0x369] = '\0';
                  pcVar15[0x36a] = '\0';
                  pcVar15[0x36b] = '\0';
                  pcVar15[0x36c] = '\0';
                  pcVar15[0x36d] = '\0';
                  pcVar15[0x36e] = '\0';
                  pcVar15[0x36f] = '\0';
                  pcVar15[0x370] = '\0';
                  pcVar15[0x371] = '\0';
                  pcVar15[0x372] = '\0';
                  pcVar15[0x373] = '\0';
                  pcVar15[0x374] = '\0';
                  pcVar15[0x375] = '\0';
                  pcVar15[0x376] = '\0';
                  pcVar15[0x377] = '\0';
                  pcVar15[0x380] = '\0';
                  pcVar15[0x381] = '\0';
                  pcVar15[0x382] = '\0';
                  pcVar15[899] = '\0';
                  pcVar15[900] = '\0';
                  pcVar15[0x385] = '\0';
                  pcVar15[0x386] = '\0';
                  pcVar15[0x387] = '\0';
                  FUN_ram_00016c40(pcVar12,*(undefined8 *)(pcVar15 + 0x50));
                  FUN_ram_0002c3c8(pcVar12);
                  iVar3 = 0x1a;
                }
                else {
                  pcVar12 = (char *)(ulonglong)uStack_1c4;
                  iVar3 = iStack_1c8;
                }
              }
              else {
                pcVar12 = (char *)(ulonglong)uStack_1bc;
                iVar3 = iStack_1c0;
              }
              iVar13 = (int)pcVar12;
            }
          }
        }
      }
      goto LAB_ram_00009d90;
    }
  }
  iVar13 = (int)local_80;
  iVar3 = 0xb;
LAB_ram_00009d90:
  param_1[1] = iVar13;
  *param_1 = iVar3;
  return;
}

// Function: FUN_ram_00009f80
void FUN_ram_00009f80(void)

{
  bool bVar1;
  int iVar2;
  longlong lVar3;
  undefined8 uVar4;
  int iVar5;
  longlong *unaff_R9;
  ulonglong local_218;
  longlong local_208;
  longlong local_200;
  undefined8 local_1f8;
  longlong local_1f0;
  undefined8 *local_1e8;
  longlong *local_1e0;
  int *local_1d8;
  longlong *local_1d0;
  int local_1c8;
  uint local_1c4;
  int local_1c0;
  uint local_1bc;
  undefined1 auStack_1b8 [39];
  undefined1 local_191;
  longlong local_130;
  longlong local_128;
  longlong local_120;
  longlong local_118;
  longlong local_110;
  longlong local_108;
  longlong local_100;
  longlong local_f8;
  longlong local_cc;
  longlong local_c4;
  longlong local_bc;
  longlong local_b4;
  undefined1 *local_98;
  undefined8 local_90;
  undefined *local_88;
  undefined8 local_80;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_18;
  undefined8 local_10;
  
  FUN_ram_00009f80();
  local_88 = &DAT_ram_00033580;
  local_80 = 0x20;
  local_90 = 0x20;
  local_10 = 0xd3bb8723dd54a054;
  local_18 = 0x6dd2523bce0a93a0;
  local_20 = 0x7a819dd33c7070c6;
  local_28 = 0xe959f7272b74fd7a;
  local_98 = auStack_1b8;
  FUN_ram_0002fa20(&local_cc,&local_98,3,&local_28);
  local_118 = local_b4;
  local_120 = local_bc;
  local_128 = local_c4;
  local_130 = local_cc;
  local_88 = &DAT_ram_00033580;
  local_80 = 0x20;
  local_90 = 0x20;
  local_10 = 0xd3bb8723dd54a054;
  local_18 = 0x6dd2523bce0a93a0;
  local_20 = 0x7a819dd33c7070c6;
  local_28 = 0xe959f7272b74fd7a;
  local_98 = auStack_1b8;
  FUN_ram_0002fa20(&local_cc,&local_98,3,&local_28);
  local_f8 = local_b4;
  local_100 = local_bc;
  local_108 = local_c4;
  local_110 = local_cc;
  lVar3 = *local_1d0;
  if ((((*unaff_R9 != *(longlong *)(lVar3 + 8)) || (unaff_R9[1] != *(longlong *)(lVar3 + 0x10))) ||
      (unaff_R9[2] != *(longlong *)(lVar3 + 0x18))) ||
     (bVar1 = false, unaff_R9[3] != *(longlong *)(lVar3 + 0x20))) {
    bVar1 = true;
  }
  iVar2 = 0;
  iVar5 = -0x5452e216;
  if (!bVar1) {
    lVar3 = *(longlong *)(local_1f0 + 0x28);
    if (((*local_1e0 != *(longlong *)(lVar3 + 8)) || (local_1e0[1] != *(longlong *)(lVar3 + 0x10)))
       || ((local_1e0[2] != *(longlong *)(lVar3 + 0x18) ||
           (bVar1 = false, local_1e0[3] != *(longlong *)(lVar3 + 0x20))))) {
      bVar1 = true;
    }
    if (!bVar1) {
      lVar3 = *(longlong *)(local_1f0 + 0x10);
      if (((local_130 != *(longlong *)(lVar3 + 8)) || (local_128 != *(longlong *)(lVar3 + 0x10))) ||
         ((local_120 != *(longlong *)(lVar3 + 0x18) ||
          (bVar1 = false, local_118 != *(longlong *)(lVar3 + 0x20))))) {
        bVar1 = true;
      }
      iVar5 = -0x4520531d;
      if (!bVar1) {
        lVar3 = *(longlong *)(local_1f0 + 0x18);
        if ((((local_cc != *(longlong *)(lVar3 + 8)) || (local_c4 != *(longlong *)(lVar3 + 0x10)))
            || (local_bc != *(longlong *)(lVar3 + 0x18))) ||
           (bVar1 = false, local_b4 != *(longlong *)(lVar3 + 0x20))) {
          bVar1 = true;
        }
        if (!bVar1) {
          FUN_ram_00008948(&local_1c0,local_1f0,(longlong *)(local_1f0 + 0x10),local_1f8);
          if (local_1c0 == 0x1a) {
            FUN_ram_00008948(&local_1c8,local_1f0,(longlong *)(local_1f0 + 0x18),local_1f8);
            if (local_1c8 == 0x1a) {
              FUN_ram_00016c40(local_218,*(undefined8 *)(local_208 + 0x50));
              FUN_ram_00031b28(local_218,local_200 + 0xf0,0x90);
              FUN_ram_00031b28(local_208 + 0xe8,local_200,0xf0);
              *(undefined8 *)(local_208 + 0x270) = local_1e8[3];
              *(undefined8 *)(local_208 + 0x268) = local_1e8[2];
              *(undefined8 *)(local_208 + 0x260) = local_1e8[1];
              *(undefined8 *)(local_208 + 600) = *local_1e8;
              *(longlong *)(local_208 + 0x210) = unaff_R9[3];
              *(longlong *)(local_208 + 0x208) = unaff_R9[2];
              *(longlong *)(local_208 + 0x200) = unaff_R9[1];
              *(longlong *)(local_208 + 0x1f8) = *unaff_R9;
              *(longlong *)(local_208 + 0x1d8) = *local_1e0;
              *(longlong *)(local_208 + 0x1e0) = local_1e0[1];
              *(longlong *)(local_208 + 0x1e8) = local_1e0[2];
              *(longlong *)(local_208 + 0x1f0) = local_1e0[3];
              *(longlong *)(local_208 + 0x238) = local_130;
              *(longlong *)(local_208 + 0x240) = local_128;
              *(longlong *)(local_208 + 0x248) = local_120;
              *(longlong *)(local_208 + 0x250) = local_118;
              *(longlong *)(local_208 + 0x218) = local_110;
              *(longlong *)(local_208 + 0x220) = local_108;
              *(longlong *)(local_208 + 0x228) = local_100;
              *(longlong *)(local_208 + 0x230) = local_f8;
              *(undefined1 *)(local_208 + 0x290) = local_191;
              *(undefined4 *)(local_208 + 0x291) = 0;
              *(undefined4 *)(local_208 + 0x294) = 0;
              *(undefined1 *)(local_208 + 0x288) = *(undefined1 *)(local_200 + 0x1f0);
              *(undefined4 *)(local_208 + 0x289) = 0;
              *(undefined4 *)(local_208 + 0x28c) = 0;
              uVar4 = *(undefined8 *)(local_200 + 0x1e0);
              *(undefined8 *)(local_208 + 0x280) = *(undefined8 *)(local_200 + 0x1e8);
              *(undefined8 *)(local_208 + 0x278) = uVar4;
              *(undefined8 *)(local_208 + 0x328) = 0;
              *(undefined8 *)(local_208 + 0x330) = 0;
              *(undefined8 *)(local_208 + 0x338) = 0;
              *(undefined8 *)(local_208 + 0x340) = 0;
              *(undefined8 *)(local_208 + 0x348) = 0;
              *(undefined8 *)(local_208 + 0x350) = 0;
              *(undefined8 *)(local_208 + 0x358) = 0;
              *(undefined8 *)(local_208 + 0x360) = 0;
              *(undefined8 *)(local_208 + 0x368) = 0;
              *(undefined8 *)(local_208 + 0x370) = 0;
              *(undefined8 *)(local_208 + 0x380) = 0;
              FUN_ram_00016c40(local_218,*(undefined8 *)(local_208 + 0x50));
              FUN_ram_0002c3c8(local_218);
              iVar2 = 0x1a;
            }
            else {
              local_218 = (ulonglong)local_1c4;
              iVar2 = local_1c8;
            }
          }
          else {
            local_218 = (ulonglong)local_1bc;
            iVar2 = local_1c0;
          }
          iVar5 = (int)local_218;
        }
      }
    }
  }
  local_1d8[1] = iVar5;
  *local_1d8 = iVar2;
  return;
}

// Function: FUN_ram_0000a860
/* WARNING: Restarted to delay deadcode elimination for space: stack */

void FUN_ram_0000a860(uint *param_1,longlong *param_2,ulonglong param_3,ulonglong param_4,
                     longlong param_5)

{
  bool bVar1;
  uint uVar2;
  longlong lVar3;
  longlong *plVar4;
  longlong *plVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  ulonglong *puVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  longlong lVar12;
  int aiStack_490 [2];
  int aiStack_488 [2];
  uint uStack_480;
  uint uStack_47c;
  uint auStack_478 [2];
  uint uStack_470;
  uint uStack_46c;
  uint auStack_468 [2];
  undefined1 auStack_460 [240];
  undefined1 auStack_370 [16];
  longlong lStack_360;
  longlong lStack_358;
  longlong lStack_330;
  longlong lStack_328;
  longlong lStack_300;
  longlong lStack_2f8;
  undefined1 auStack_2e0 [80];
  longlong local_290;
  longlong local_288;
  longlong local_280;
  longlong local_278;
  longlong local_270;
  longlong local_268;
  longlong local_260;
  longlong local_258;
  longlong local_250;
  longlong local_248;
  longlong local_240;
  longlong local_238;
  longlong local_230;
  longlong local_228;
  longlong local_220;
  longlong local_218;
  undefined8 uStack_210;
  undefined8 uStack_208;
  undefined8 uStack_200;
  undefined8 uStack_1f8;
  undefined8 uStack_1f0;
  undefined8 uStack_1e8;
  undefined8 uStack_1e0;
  undefined8 uStack_1d8;
  undefined2 uStack_1d0;
  byte bStack_1ce;
  longlong **local_1c0;
  longlong local_1b8;
  longlong local_1b0;
  longlong local_1a8;
  undefined1 local_199;
  undefined *local_198;
  undefined8 local_190;
  undefined8 *local_188;
  undefined8 local_180;
  longlong *local_178;
  undefined8 local_170;
  longlong *local_168;
  undefined8 local_160;
  undefined1 *local_158;
  undefined8 local_150;
  undefined **local_148;
  undefined8 local_140;
  longlong local_138;
  undefined2 local_130;
  longlong local_128;
  undefined2 local_120;
  undefined4 local_118;
  undefined4 local_114;
  undefined4 uStack_110;
  undefined4 local_10c;
  undefined4 uStack_108;
  undefined4 local_104;
  undefined4 uStack_100;
  undefined4 local_fc;
  undefined4 uStack_f8;
  undefined8 local_f4;
  undefined8 local_ec;
  longlong *local_e0;
  ulonglong *local_d8;
  longlong **local_d0;
  longlong local_c8;
  longlong local_c0;
  longlong local_b8;
  undefined1 local_b0;
  longlong **local_60;
  undefined8 **local_58;
  longlong *local_50;
  undefined4 *local_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 **local_28;
  longlong ***local_20;
  undefined8 local_18;
  undefined8 local_10;
  undefined8 local_8;
  
  uVar2 = 0;
  uVar11 = 0xbadc0de1;
  if (param_3 < 10) goto LAB_ram_0000b768;
  lVar12 = *param_2;
  uVar11 = 0xabad1dea;
  if (*(char *)(lVar12 + 1) == '\0') goto LAB_ram_0000b768;
  if ((((*(longlong *)(lVar12 + 8) != -0x32b20de6c4775e40) ||
       (*(longlong *)(lVar12 + 0x10) != -0x15200f0917dbfd5f)) ||
      (*(longlong *)(lVar12 + 0x18) != -0x47ebde6f8ba50bd0)) ||
     (bVar1 = false, *(longlong *)(lVar12 + 0x20) != 0x4164f987ea077494)) {
    bVar1 = true;
  }
  if (bVar1) goto LAB_ram_0000b768;
  if (param_5 != 0x2a0) {
                    /* WARNING: Subroutine does not return */
    FUN_ram_000011b0(&DAT_ram_000337df,10,2);
  }
  if ((param_4 & 7) != 0) {
                    /* WARNING: Subroutine does not return */
    FUN_ram_000011b0(&DAT_ram_000337df,10,0);
  }
  FUN_ram_00031b28(auStack_460,param_4,0x2a0);
  lVar7 = param_2[7];
  if (((*(longlong *)(lVar7 + 8) != -0x6c5e9a281e0922fa) ||
      (*(longlong *)(lVar7 + 0x10) != -0x53861431b91e3427)) ||
     ((*(longlong *)(lVar7 + 0x18) != -0x6ec8a4a0127a4be4 ||
      (bVar1 = false, *(longlong *)(lVar7 + 0x20) != -0x56ff00817a0a73c6)))) {
    bVar1 = true;
  }
  lVar3 = param_2[8];
  if (bVar1) {
    if (((*(longlong *)(lVar7 + 8) != -0x21708a111e0922fa) ||
        (*(longlong *)(lVar7 + 0x10) != -0x2532931b43a2bde8)) ||
       ((*(longlong *)(lVar7 + 0x18) != 0x270db9834dfc1ab6 ||
        (bVar1 = false, *(longlong *)(lVar7 + 0x20) != -0x3745e27d7064202)))) {
      bVar1 = true;
    }
    if (!bVar1) goto LAB_ram_0000aaf0;
  }
  else {
LAB_ram_0000aaf0:
    if ((((*(longlong *)(lVar3 + 8) != -0x6c5e9a281e0922fa) ||
         (*(longlong *)(lVar3 + 0x10) != -0x53861431b91e3427)) ||
        (*(longlong *)(lVar3 + 0x18) != -0x6ec8a4a0127a4be4)) ||
       (bVar1 = false, *(longlong *)(lVar3 + 0x20) != -0x56ff00817a0a73c6)) {
      bVar1 = true;
    }
    if ((!bVar1) ||
       (((*(longlong *)(lVar3 + 8) == -0x21708a111e0922fa &&
         (*(longlong *)(lVar3 + 0x10) == -0x2532931b43a2bde8)) &&
        ((*(longlong *)(lVar3 + 0x18) == 0x270db9834dfc1ab6 &&
         (*(longlong *)(lVar3 + 0x20) == -0x3745e27d7064202)))))) {
      lVar7 = param_2[2];
      if (((*(longlong *)(lVar7 + 8) != local_270) || (*(longlong *)(lVar7 + 0x10) != local_268)) ||
         ((*(longlong *)(lVar7 + 0x18) != local_260 ||
          (bVar1 = false, *(longlong *)(lVar7 + 0x20) != local_258)))) {
        bVar1 = true;
      }
      uVar2 = 0;
      uVar11 = 0xbadc0de4;
      if (bVar1) goto LAB_ram_0000b768;
      lVar3 = param_2[3];
      if ((((*(longlong *)(lVar3 + 8) != local_290) || (*(longlong *)(lVar3 + 0x10) != local_288))
          || (*(longlong *)(lVar3 + 0x18) != local_280)) ||
         (bVar1 = false, *(longlong *)(lVar3 + 0x20) != local_278)) {
        bVar1 = true;
      }
      if (bVar1) goto LAB_ram_0000b768;
      plVar4 = param_2 + 4;
      lVar8 = *plVar4;
      if (((*(longlong *)(lVar8 + 8) != local_230) || (*(longlong *)(lVar8 + 0x10) != local_228)) ||
         ((*(longlong *)(lVar8 + 0x18) != local_220 ||
          (bVar1 = false, *(longlong *)(lVar8 + 0x20) != local_218)))) {
        bVar1 = true;
      }
      uVar11 = 0xbadc0de5;
      if (bVar1) goto LAB_ram_0000b768;
      plVar5 = param_2 + 5;
      lVar6 = *plVar5;
      if (((*(longlong *)(lVar6 + 8) != local_250) || (*(longlong *)(lVar6 + 0x10) != local_248)) ||
         ((*(longlong *)(lVar6 + 0x18) != local_240 ||
          (bVar1 = false, *(longlong *)(lVar6 + 0x20) != local_238)))) {
        bVar1 = true;
      }
      if (bVar1) goto LAB_ram_0000b768;
      puVar9 = (ulonglong *)(param_2 + 1);
      FUN_ram_000085e0(&local_d0);
      local_1a8 = local_b8;
      local_1b0 = local_c0;
      local_1b8 = local_c8;
      local_1c0 = local_d0;
      local_199 = local_b0;
      uVar10 = *puVar9;
      local_128 = uVar10 + 8;
      if ((((local_d0 != *(longlong ***)(uVar10 + 8)) || (local_c8 != *(longlong *)(uVar10 + 0x10)))
          || (local_c0 != *(longlong *)(uVar10 + 0x18))) ||
         (bVar1 = false, local_b8 != *(longlong *)(uVar10 + 0x20))) {
        bVar1 = true;
      }
      if (bVar1) {
        uVar2 = 0;
        uVar11 = 0xbadc0de7;
        goto LAB_ram_0000b768;
      }
      local_158 = &local_199;
      local_198 = &DAT_ram_00033bb2;
      local_150 = 1;
      local_160 = 0x20;
      local_170 = 0x20;
      local_180 = 0x20;
      local_190 = 6;
      local_148 = &local_198;
      local_140 = 5;
      local_120 = 0x101;
      local_130 = 0x101;
      local_104 = 0x2b74fd7a;
      uStack_100 = 0xe959f727;
      local_fc = 0x3c7070c6;
      uStack_f8 = 0x7a819dd3;
      local_f4 = 0x6dd2523bce0a93a0;
      local_ec = 0xd3bb8723dd54a054;
      local_10c = 0x6c0;
      uStack_108 = 0;
      local_114 = 0xc51c00;
      uStack_110 = 0;
      local_118 = 0;
      local_20 = &local_60;
      local_28 = &local_d0;
      local_48 = &local_118;
      local_50 = &local_138;
      local_58 = &local_d0;
      local_60 = &local_e0;
      local_8 = 2;
      local_10 = 2;
      local_18 = 0;
      local_30 = 2;
      local_38 = 2;
      local_40 = 0;
      local_188 = &uStack_210;
      local_178 = &local_270;
      local_168 = &local_290;
      local_138 = lVar12 + 8;
      local_e0 = param_2;
      local_d8 = puVar9;
      uVar2 = FUN_ram_00000278(&local_60);
      uVar11 = uVar10;
      if (uVar2 != 0x1a) goto LAB_ram_0000b768;
      local_48 = &local_118;
      local_58 = (undefined8 **)&local_138;
      local_60 = (longlong **)&DAT_ram_000335a0;
      local_40 = 0x34;
      local_50 = (longlong *)0x2;
      FUN_ram_0000b380(&local_60,&local_d0,2,&local_148,1);
      if (((*(longlong *)(lVar7 + 0x28) != -0x21708a111e0922fa) ||
          (*(longlong *)(lVar7 + 0x30) != -0x2532931b43a2bde8)) ||
         ((*(longlong *)(lVar7 + 0x38) != 0x270db9834dfc1ab6 ||
          (bVar1 = false, *(longlong *)(lVar7 + 0x40) != -0x3745e27d7064202)))) {
        bVar1 = true;
      }
      if (bVar1) {
        FUN_ram_000086c8(&local_d0,&local_1c0,&local_270);
      }
      else {
        FUN_ram_00008808(&local_d0,&local_1c0,&local_270);
      }
      uStack_100 = (undefined4)local_b8;
      local_fc = (undefined4)((ulonglong)local_b8 >> 0x20);
      uStack_108 = (undefined4)local_c0;
      local_104 = (undefined4)((ulonglong)local_c0 >> 0x20);
      uStack_110 = (undefined4)local_c8;
      local_10c = (undefined4)((ulonglong)local_c8 >> 0x20);
      local_118 = SUB84(local_d0,0);
      local_114 = (undefined4)((ulonglong)local_d0 >> 0x20);
      if (((*(longlong *)(lVar3 + 0x28) != -0x21708a111e0922fa) ||
          (*(longlong *)(lVar3 + 0x30) != -0x2532931b43a2bde8)) ||
         ((*(longlong *)(lVar3 + 0x38) != 0x270db9834dfc1ab6 ||
          (bVar1 = false, *(longlong *)(lVar3 + 0x40) != -0x3745e27d7064202)))) {
        bVar1 = true;
      }
      if (bVar1) {
        FUN_ram_000086c8(&local_d0,&local_1c0,&local_290);
      }
      else {
        FUN_ram_00008808(&local_d0,&local_1c0,&local_290);
      }
      local_48 = (undefined4 *)local_b8;
      local_50 = (longlong *)local_c0;
      local_58 = (undefined8 **)local_c8;
      local_60 = local_d0;
      if ((((CONCAT44(local_114,local_118) != *(longlong *)(lVar8 + 8)) ||
           (CONCAT44(local_10c,uStack_110) != *(longlong *)(lVar8 + 0x10))) ||
          (CONCAT44(local_104,uStack_108) != *(longlong *)(lVar8 + 0x18))) ||
         (bVar1 = false, CONCAT44(local_fc,uStack_100) != *(longlong *)(lVar8 + 0x20))) {
        bVar1 = true;
      }
      uVar11 = 0xbadc0de8;
      if (!bVar1) {
        if (((local_d0 != *(longlong ***)(lVar6 + 8)) || (local_c8 != *(longlong *)(lVar6 + 0x10)))
           || ((local_c0 != *(longlong *)(lVar6 + 0x18) ||
               (bVar1 = false, local_b8 != *(longlong *)(lVar6 + 0x20))))) {
          bVar1 = true;
        }
        if (!bVar1) {
          if (((*(longlong *)(lVar7 + 0x28) != -0x6c5e9a281e0922fa) ||
              (*(longlong *)(lVar7 + 0x30) != -0x53861431b91e3427)) ||
             ((*(longlong *)(lVar7 + 0x38) != -0x6ec8a4a0127a4be4 ||
              (bVar1 = false, *(longlong *)(lVar7 + 0x40) != -0x56ff00817a0a73c6)))) {
            bVar1 = true;
          }
          if (bVar1) {
            if ((((*(longlong *)(lVar7 + 0x28) != -0x21708a111e0922fa) ||
                 (*(longlong *)(lVar7 + 0x30) != -0x2532931b43a2bde8)) ||
                (*(longlong *)(lVar7 + 0x38) != 0x270db9834dfc1ab6)) ||
               (bVar1 = false, *(longlong *)(lVar7 + 0x40) != -0x3745e27d7064202)) {
              bVar1 = true;
            }
            uVar11 = 0xbadc0de3;
            if (bVar1) goto LAB_ram_0000b758;
            FUN_ram_00008e90(auStack_468,param_2,plVar4,puVar9);
            uVar2 = auStack_468[0];
            if (auStack_468[0] != 0x1a) goto LAB_ram_0000b768;
          }
          else {
            FUN_ram_00008948(&uStack_470,param_2,plVar4,puVar9);
            if (uStack_470 != 0x1a) {
              uVar11 = (ulonglong)uStack_46c;
              uVar2 = uStack_470;
              goto LAB_ram_0000b768;
            }
          }
          if (((*(longlong *)(lVar3 + 0x28) != -0x6c5e9a281e0922fa) ||
              (*(longlong *)(lVar3 + 0x30) != -0x53861431b91e3427)) ||
             ((*(longlong *)(lVar3 + 0x38) != -0x6ec8a4a0127a4be4 ||
              (bVar1 = false, *(longlong *)(lVar3 + 0x40) != -0x56ff00817a0a73c6)))) {
            bVar1 = true;
          }
          if (bVar1) {
            if (((*(longlong *)(lVar3 + 0x28) != -0x21708a111e0922fa) ||
                (*(longlong *)(lVar3 + 0x30) != -0x2532931b43a2bde8)) ||
               ((*(longlong *)(lVar3 + 0x38) != 0x270db9834dfc1ab6 ||
                (bVar1 = false, *(longlong *)(lVar3 + 0x40) != -0x3745e27d7064202)))) {
              bVar1 = true;
            }
            uVar11 = 0xbadc0de3;
            if (bVar1) goto LAB_ram_0000b758;
            FUN_ram_00008e90(auStack_478,param_2,plVar5,puVar9);
            uVar2 = auStack_478[0];
            if (auStack_478[0] != 0x1a) goto LAB_ram_0000b768;
          }
          else {
            FUN_ram_00008948(&uStack_480,param_2,plVar5,puVar9);
            if (uStack_480 != 0x1a) {
              uVar11 = (ulonglong)uStack_47c;
              uVar2 = uStack_480;
              goto LAB_ram_0000b768;
            }
          }
          uVar11 = 0xbadc0de9;
          if ((((bStack_1ce < 3) && (uVar11 = 0xbadc0dea, lStack_360 <= lStack_358)) &&
              (lStack_358 <= lStack_330)) &&
             (((lStack_330 <= lStack_328 && (lStack_328 <= lStack_300)) &&
              (lStack_300 <= lStack_2f8)))) {
            FUN_ram_00016cc0(aiStack_488,auStack_460);
            uVar11 = 0xbadc0deb;
            if (aiStack_488[0] == 0x1a) {
              FUN_ram_00016a10(aiStack_490);
              uVar2 = 0;
              uVar11 = 0xbadc0dec;
              if (aiStack_490[0] == 0x1a) {
                uVar11 = uVar10 + 0x58;
                FUN_ram_00016c40(uVar11,*(undefined8 *)(uVar10 + 0x50));
                FUN_ram_00031b48(uVar11,auStack_370,0x90);
                FUN_ram_00031b48(uVar10 + 0xe8,auStack_460,0xf0);
                *(longlong *)(uVar10 + 0x1d8) = local_290;
                *(longlong *)(uVar10 + 0x1f0) = local_278;
                *(longlong *)(uVar10 + 0x1e8) = local_280;
                *(longlong *)(uVar10 + 0x1e0) = local_288;
                *(longlong *)(uVar10 + 0x210) = local_258;
                *(longlong *)(uVar10 + 0x208) = local_260;
                *(longlong *)(uVar10 + 0x200) = local_268;
                *(longlong *)(uVar10 + 0x1f8) = local_270;
                *(longlong *)(uVar10 + 0x230) = local_238;
                *(longlong *)(uVar10 + 0x228) = local_240;
                *(longlong *)(uVar10 + 0x220) = local_248;
                *(longlong *)(uVar10 + 0x218) = local_250;
                *(longlong *)(uVar10 + 0x250) = local_218;
                *(longlong *)(uVar10 + 0x248) = local_220;
                *(longlong *)(uVar10 + 0x240) = local_228;
                *(longlong *)(uVar10 + 0x238) = local_230;
                *(undefined8 *)(uVar10 + 0x270) = uStack_1f8;
                *(undefined8 *)(uVar10 + 0x268) = uStack_200;
                *(undefined8 *)(uVar10 + 0x260) = uStack_208;
                *(undefined8 *)(uVar10 + 600) = uStack_210;
                *(undefined8 *)(uVar10 + 0x278) = uStack_1f0;
                *(undefined8 *)(uVar10 + 0x280) = uStack_1e8;
                *(byte *)(uVar10 + 0x288) = bStack_1ce;
                *(undefined4 *)(uVar10 + 0x28c) = 0;
                *(undefined4 *)(uVar10 + 0x289) = 0;
                *(undefined1 *)(uVar10 + 0x290) = local_199;
                *(undefined8 *)(uVar10 + 0x310) = 0;
                *(undefined8 *)(uVar10 + 0x308) = 0;
                *(undefined8 *)(uVar10 + 0x300) = 0;
                *(undefined8 *)(uVar10 + 0x2f8) = 0;
                *(undefined8 *)(uVar10 + 0x2f0) = 0;
                *(undefined8 *)(uVar10 + 0x2e8) = 0;
                *(undefined8 *)(uVar10 + 0x2e0) = 0;
                *(undefined8 *)(uVar10 + 0x2d8) = 0;
                *(undefined8 *)(uVar10 + 0x2d0) = 0;
                *(undefined8 *)(uVar10 + 0x2c8) = 0;
                *(undefined8 *)(uVar10 + 0x2c0) = 0;
                *(undefined8 *)(uVar10 + 0x2b8) = 0;
                *(undefined8 *)(uVar10 + 0x2b0) = 0;
                *(undefined8 *)(uVar10 + 0x2a8) = 0;
                *(undefined8 *)(uVar10 + 0x2a0) = 0;
                *(undefined8 *)(uVar10 + 0x298) = 0;
                *(undefined4 *)(uVar10 + 0x294) = 0;
                *(undefined4 *)(uVar10 + 0x291) = 0;
                *(undefined8 *)(uVar10 + 800) = uStack_1e0;
                *(undefined2 *)(uVar10 + 0x318) = uStack_1d0;
                *(undefined4 *)(uVar10 + 0x31a) = 0;
                *(undefined2 *)(uVar10 + 0x31e) = 0;
                FUN_ram_00031b28(uVar10 + 0x328,auStack_2e0,0x50);
                *(undefined8 *)(uVar10 + 0x378) = uStack_1d8;
                *(undefined8 *)(uVar10 + 0x400) = 0;
                *(undefined8 *)(uVar10 + 0x3f8) = 0;
                *(undefined8 *)(uVar10 + 0x3f0) = 0;
                *(undefined8 *)(uVar10 + 1000) = 0;
                *(undefined8 *)(uVar10 + 0x3e0) = 0;
                *(undefined8 *)(uVar10 + 0x3d8) = 0;
                *(undefined8 *)(uVar10 + 0x3d0) = 0;
                *(undefined8 *)(uVar10 + 0x3c8) = 0;
                *(undefined8 *)(uVar10 + 0x3c0) = 0;
                *(undefined8 *)(uVar10 + 0x3b8) = 0;
                *(undefined8 *)(uVar10 + 0x3b0) = 0;
                *(undefined8 *)(uVar10 + 0x3a8) = 0;
                *(undefined8 *)(uVar10 + 0x3a0) = 0;
                *(undefined8 *)(uVar10 + 0x398) = 0;
                *(undefined8 *)(uVar10 + 0x390) = 0;
                *(undefined8 *)(uVar10 + 0x388) = 0;
                *(undefined8 *)(uVar10 + 0x380) = 0;
                FUN_ram_00031b68(uVar10 + 0x408,0,0x308);
                *(undefined8 *)(uVar10 + 0x710) = 4;
                FUN_ram_0002c3c8(uVar11);
                uVar2 = 0x1a;
              }
              goto LAB_ram_0000b768;
            }
          }
        }
      }
LAB_ram_0000b758:
      uVar2 = 0;
      goto LAB_ram_0000b768;
    }
  }
  uVar11 = 0xbadc0de3;
  uVar2 = 0;
LAB_ram_0000b768:
  param_1[1] = (uint)uVar11;
  *param_1 = uVar2;
  return;
}

// Function: FUN_ram_0000b380
void FUN_ram_0000b380(void)

{
  bool bVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong unaff_R7;
  ulonglong uVar5;
  undefined8 *local_4f0;
  undefined8 *local_4e0;
  undefined8 *local_4d8;
  longlong local_4c8;
  longlong local_4b8;
  longlong local_4a8;
  longlong local_4a0;
  int *local_498;
  int local_490 [2];
  int local_488 [2];
  int local_480;
  uint local_47c;
  int local_478 [2];
  int local_470;
  uint local_46c;
  int local_468 [2];
  undefined1 auStack_460 [240];
  undefined1 auStack_370 [16];
  longlong local_360;
  longlong local_358;
  longlong local_330;
  longlong local_328;
  longlong local_300;
  longlong local_2f8;
  undefined1 auStack_2e0 [144];
  undefined8 local_250;
  undefined8 uStack_248;
  undefined8 uStack_240;
  undefined8 uStack_238;
  undefined8 local_230;
  undefined8 uStack_228;
  undefined8 uStack_220;
  undefined8 uStack_218;
  undefined8 local_1f0;
  undefined8 local_1e8;
  undefined8 local_1e0;
  undefined8 local_1d8;
  undefined2 local_1d0;
  byte local_1ce;
  undefined1 auStack_1c0 [39];
  undefined1 local_199;
  longlong local_118;
  longlong local_110;
  longlong local_108;
  longlong local_100;
  longlong local_d0;
  longlong local_c8;
  longlong local_c0;
  longlong local_b8;
  longlong local_60;
  longlong local_58;
  longlong local_50;
  longlong local_48;
  
  FUN_ram_0000b380();
  if ((((*(longlong *)(local_4a0 + 0x28) != -0x21708a111e0922fa) ||
       (*(longlong *)(local_4a0 + 0x30) != -0x2532931b43a2bde8)) ||
      (*(longlong *)(local_4a0 + 0x38) != 0x270db9834dfc1ab6)) ||
     (bVar1 = false, *(longlong *)(local_4a0 + 0x40) != -0x3745e27d7064202)) {
    bVar1 = true;
  }
  if (bVar1) {
    FUN_ram_000086c8(&local_d0,auStack_1c0,local_4d8);
  }
  else {
    FUN_ram_00008808(&local_d0,auStack_1c0,local_4d8);
  }
  local_100 = local_b8;
  local_108 = local_c0;
  local_110 = local_c8;
  local_118 = local_d0;
  if (((*(longlong *)(local_4a8 + 0x28) != -0x21708a111e0922fa) ||
      (*(longlong *)(local_4a8 + 0x30) != -0x2532931b43a2bde8)) ||
     ((*(longlong *)(local_4a8 + 0x38) != 0x270db9834dfc1ab6 ||
      (bVar1 = false, *(longlong *)(local_4a8 + 0x40) != -0x3745e27d7064202)))) {
    bVar1 = true;
  }
  if (bVar1) {
    FUN_ram_000086c8(&local_d0,auStack_1c0,local_4e0);
  }
  else {
    FUN_ram_00008808(&local_d0,auStack_1c0,local_4e0);
  }
  local_48 = local_b8;
  local_50 = local_c0;
  local_58 = local_c8;
  local_60 = local_d0;
  if (((local_118 != *(longlong *)(local_4b8 + 8)) || (local_110 != *(longlong *)(local_4b8 + 0x10))
      ) || ((local_108 != *(longlong *)(local_4b8 + 0x18) ||
            (bVar1 = false, local_100 != *(longlong *)(local_4b8 + 0x20))))) {
    bVar1 = true;
  }
  uVar5 = 0xbadc0de8;
  if (!bVar1) {
    if ((((local_d0 != *(longlong *)(local_4c8 + 8)) ||
         (local_c8 != *(longlong *)(local_4c8 + 0x10))) ||
        (local_c0 != *(longlong *)(local_4c8 + 0x18))) ||
       (bVar1 = false, local_b8 != *(longlong *)(local_4c8 + 0x20))) {
      bVar1 = true;
    }
    if (!bVar1) {
      if (((*(longlong *)(local_4a0 + 0x28) != -0x6c5e9a281e0922fa) ||
          (*(longlong *)(local_4a0 + 0x30) != -0x53861431b91e3427)) ||
         ((*(longlong *)(local_4a0 + 0x38) != -0x6ec8a4a0127a4be4 ||
          (bVar1 = false, *(longlong *)(local_4a0 + 0x40) != -0x56ff00817a0a73c6)))) {
        bVar1 = true;
      }
      if (bVar1) {
        if (((*(longlong *)(local_4a0 + 0x28) != -0x21708a111e0922fa) ||
            (*(longlong *)(local_4a0 + 0x30) != -0x2532931b43a2bde8)) ||
           ((*(longlong *)(local_4a0 + 0x38) != 0x270db9834dfc1ab6 ||
            (bVar1 = false, *(longlong *)(local_4a0 + 0x40) != -0x3745e27d7064202)))) {
          bVar1 = true;
        }
        uVar5 = 0xbadc0de3;
        if (bVar1) goto LAB_ram_0000b758;
        FUN_ram_00008e90(local_468);
        if (local_468[0] != 0x1a) goto LAB_ram_0000b768;
      }
      else {
        FUN_ram_00008948(&local_470);
        if (local_470 != 0x1a) {
          uVar5 = (ulonglong)local_46c;
          local_468[0] = local_470;
          goto LAB_ram_0000b768;
        }
      }
      if ((((*(longlong *)(local_4a8 + 0x28) != -0x6c5e9a281e0922fa) ||
           (*(longlong *)(local_4a8 + 0x30) != -0x53861431b91e3427)) ||
          (*(longlong *)(local_4a8 + 0x38) != -0x6ec8a4a0127a4be4)) ||
         (bVar1 = false, *(longlong *)(local_4a8 + 0x40) != -0x56ff00817a0a73c6)) {
        bVar1 = true;
      }
      if (bVar1) {
        if (((*(longlong *)(local_4a8 + 0x28) != -0x21708a111e0922fa) ||
            (*(longlong *)(local_4a8 + 0x30) != -0x2532931b43a2bde8)) ||
           ((*(longlong *)(local_4a8 + 0x38) != 0x270db9834dfc1ab6 ||
            (bVar1 = false, *(longlong *)(local_4a8 + 0x40) != -0x3745e27d7064202)))) {
          bVar1 = true;
        }
        uVar5 = 0xbadc0de3;
        if (bVar1) goto LAB_ram_0000b758;
        FUN_ram_00008e90(local_478);
        local_468[0] = local_478[0];
        if (local_478[0] != 0x1a) goto LAB_ram_0000b768;
      }
      else {
        FUN_ram_00008948(&local_480);
        if (local_480 != 0x1a) {
          uVar5 = (ulonglong)local_47c;
          local_468[0] = local_480;
          goto LAB_ram_0000b768;
        }
      }
      uVar5 = 0xbadc0de9;
      if (((local_1ce < 3) && (uVar5 = 0xbadc0dea, local_360 <= local_358)) &&
         ((local_358 <= local_330 &&
          (((local_330 <= local_328 && (local_328 <= local_300)) && (local_300 <= local_2f8)))))) {
        FUN_ram_00016cc0(local_488,auStack_460);
        uVar5 = 0xbadc0deb;
        if (local_488[0] == 0x1a) {
          FUN_ram_00016a10(local_490);
          uVar5 = 0xbadc0dec;
          local_468[0] = 0;
          if (local_490[0] == 0x1a) {
            uVar5 = unaff_R7 + 0x58;
            FUN_ram_00016c40(uVar5,*(undefined8 *)(unaff_R7 + 0x50));
            FUN_ram_00031b48(uVar5,auStack_370,0x90);
            FUN_ram_00031b48(unaff_R7 + 0xe8,auStack_460,0xf0);
            uVar2 = local_4e0[1];
            uVar3 = local_4e0[2];
            uVar4 = local_4e0[3];
            *(undefined8 *)(unaff_R7 + 0x1d8) = *local_4e0;
            *(undefined8 *)(unaff_R7 + 0x1f0) = uVar4;
            *(undefined8 *)(unaff_R7 + 0x1e8) = uVar3;
            *(undefined8 *)(unaff_R7 + 0x1e0) = uVar2;
            uVar2 = *local_4d8;
            uVar3 = local_4d8[1];
            uVar4 = local_4d8[2];
            *(undefined8 *)(unaff_R7 + 0x210) = local_4d8[3];
            *(undefined8 *)(unaff_R7 + 0x208) = uVar4;
            *(undefined8 *)(unaff_R7 + 0x200) = uVar3;
            *(undefined8 *)(unaff_R7 + 0x1f8) = uVar2;
            *(undefined8 *)(unaff_R7 + 0x230) = uStack_238;
            *(undefined8 *)(unaff_R7 + 0x228) = uStack_240;
            *(undefined8 *)(unaff_R7 + 0x220) = uStack_248;
            *(undefined8 *)(unaff_R7 + 0x218) = local_250;
            *(undefined8 *)(unaff_R7 + 0x250) = uStack_218;
            *(undefined8 *)(unaff_R7 + 0x248) = uStack_220;
            *(undefined8 *)(unaff_R7 + 0x240) = uStack_228;
            *(undefined8 *)(unaff_R7 + 0x238) = local_230;
            uVar2 = *local_4f0;
            uVar3 = local_4f0[1];
            uVar4 = local_4f0[2];
            *(undefined8 *)(unaff_R7 + 0x270) = local_4f0[3];
            *(undefined8 *)(unaff_R7 + 0x268) = uVar4;
            *(undefined8 *)(unaff_R7 + 0x260) = uVar3;
            *(undefined8 *)(unaff_R7 + 600) = uVar2;
            *(undefined8 *)(unaff_R7 + 0x278) = local_1f0;
            *(undefined8 *)(unaff_R7 + 0x280) = local_1e8;
            *(byte *)(unaff_R7 + 0x288) = local_1ce;
            *(undefined4 *)(unaff_R7 + 0x28c) = 0;
            *(undefined4 *)(unaff_R7 + 0x289) = 0;
            *(undefined1 *)(unaff_R7 + 0x290) = local_199;
            *(undefined8 *)(unaff_R7 + 0x310) = 0;
            *(undefined8 *)(unaff_R7 + 0x308) = 0;
            *(undefined8 *)(unaff_R7 + 0x300) = 0;
            *(undefined8 *)(unaff_R7 + 0x2f8) = 0;
            *(undefined8 *)(unaff_R7 + 0x2f0) = 0;
            *(undefined8 *)(unaff_R7 + 0x2e8) = 0;
            *(undefined8 *)(unaff_R7 + 0x2e0) = 0;
            *(undefined8 *)(unaff_R7 + 0x2d8) = 0;
            *(undefined8 *)(unaff_R7 + 0x2d0) = 0;
            *(undefined8 *)(unaff_R7 + 0x2c8) = 0;
            *(undefined8 *)(unaff_R7 + 0x2c0) = 0;
            *(undefined8 *)(unaff_R7 + 0x2b8) = 0;
            *(undefined8 *)(unaff_R7 + 0x2b0) = 0;
            *(undefined8 *)(unaff_R7 + 0x2a8) = 0;
            *(undefined8 *)(unaff_R7 + 0x2a0) = 0;
            *(undefined8 *)(unaff_R7 + 0x298) = 0;
            *(undefined4 *)(unaff_R7 + 0x294) = 0;
            *(undefined4 *)(unaff_R7 + 0x291) = 0;
            *(undefined8 *)(unaff_R7 + 800) = local_1e0;
            *(undefined2 *)(unaff_R7 + 0x318) = local_1d0;
            *(undefined4 *)(unaff_R7 + 0x31a) = 0;
            *(undefined2 *)(unaff_R7 + 0x31e) = 0;
            FUN_ram_00031b28(unaff_R7 + 0x328,auStack_2e0,0x50);
            *(undefined8 *)(unaff_R7 + 0x378) = local_1d8;
            *(undefined8 *)(unaff_R7 + 0x400) = 0;
            *(undefined8 *)(unaff_R7 + 0x3f8) = 0;
            *(undefined8 *)(unaff_R7 + 0x3f0) = 0;
            *(undefined8 *)(unaff_R7 + 1000) = 0;
            *(undefined8 *)(unaff_R7 + 0x3e0) = 0;
            *(undefined8 *)(unaff_R7 + 0x3d8) = 0;
            *(undefined8 *)(unaff_R7 + 0x3d0) = 0;
            *(undefined8 *)(unaff_R7 + 0x3c8) = 0;
            *(undefined8 *)(unaff_R7 + 0x3c0) = 0;
            *(undefined8 *)(unaff_R7 + 0x3b8) = 0;
            *(undefined8 *)(unaff_R7 + 0x3b0) = 0;
            *(undefined8 *)(unaff_R7 + 0x3a8) = 0;
            *(undefined8 *)(unaff_R7 + 0x3a0) = 0;
            *(undefined8 *)(unaff_R7 + 0x398) = 0;
            *(undefined8 *)(unaff_R7 + 0x390) = 0;
            *(undefined8 *)(unaff_R7 + 0x388) = 0;
            *(undefined8 *)(unaff_R7 + 0x380) = 0;
            FUN_ram_00031b68(unaff_R7 + 0x408,0,0x308);
            *(undefined8 *)(unaff_R7 + 0x710) = 4;
            FUN_ram_0002c3c8(uVar5);
            local_468[0] = 0x1a;
          }
          goto LAB_ram_0000b768;
        }
      }
    }
  }
LAB_ram_0000b758:
  local_468[0] = 0;
LAB_ram_0000b768:
  local_498[1] = (int)uVar5;
  *local_498 = local_468[0];
  return;
}

// Function: FUN_ram_0000c220
/* WARNING: Type propagation algorithm not settling */

undefined8
FUN_ram_0000c220(undefined8 *param_1,undefined8 *param_2,undefined8 *param_3,undefined8 *param_4,
                longlong param_5)

{
  char cVar1;
  char cVar2;
  byte bVar3;
  byte bVar4;
  undefined8 uVar5;
  byte *pbVar6;
  undefined1 local_152;
  undefined8 local_151;
  byte local_149;
  char *local_148;
  undefined2 local_140;
  byte *local_138;
  undefined2 local_130;
  char *local_128;
  undefined2 local_120;
  byte *local_118;
  undefined2 local_110;
  char *local_108;
  char *local_100;
  undefined8 local_f8;
  char *local_f0;
  char *local_e8;
  undefined8 local_e0;
  undefined1 local_d8;
  undefined1 local_d7;
  undefined1 local_d6;
  byte *local_d0;
  byte *local_c8;
  ulonglong local_c0;
  byte *local_b8;
  byte *local_b0;
  undefined8 local_a8;
  undefined1 local_a0;
  undefined1 local_9f;
  undefined1 local_9e;
  char *local_98;
  char *local_90;
  undefined8 local_88;
  char *local_80;
  char *local_78;
  undefined8 local_70;
  undefined1 local_68;
  undefined1 local_67;
  undefined1 local_66;
  byte *local_60;
  byte *local_58;
  undefined8 local_50;
  byte *local_48;
  byte *local_40;
  undefined8 local_38;
  undefined1 local_30;
  undefined1 local_2f;
  undefined1 local_2e;
  undefined *local_28;
  char **local_20;
  undefined8 local_18;
  undefined1 *local_10;
  undefined8 local_8;
  
  local_c8 = (byte *)*param_4;
  local_c0 = *(ulonglong *)(local_c8 + 0x50);
  if (local_c0 < 0x2c) {
    return 0;
  }
  if (local_c0 == 0x2c) {
                    /* WARNING: Subroutine does not return */
    FUN_ram_0002fc40(0x2c,0x2c,&DAT_ram_00034228);
  }
  local_151 = *(undefined8 *)(param_5 + -0x1000);
  local_149 = local_c8[0x84];
  local_152 = 0xc;
  local_100 = (char *)*param_1;
  local_90 = (char *)*param_2;
  pbVar6 = (byte *)*param_3;
  local_138 = local_c8 + 8;
  local_118 = pbVar6 + 8;
  local_128 = local_90 + 8;
  local_148 = local_100 + 8;
  local_110 = 0x100;
  local_120 = 1;
  local_130 = 0;
  local_140 = 1;
  local_108 = local_148;
  local_d0 = local_138;
  local_98 = local_128;
  local_60 = local_118;
  if (*(longlong *)(param_5 + -0xff8) != 0) {
    if (*local_100 != -1) {
      return 0xb;
    }
    if (local_100[1] == '\0') {
      cVar1 = local_100[2];
    }
    else {
      cVar1 = local_100[2];
    }
    local_d8 = local_100[1] != '\0';
    if (cVar1 == '\0') {
      cVar2 = local_100[3];
    }
    else {
      cVar2 = local_100[3];
    }
    local_d6 = cVar2 != '\0';
    local_d7 = cVar1 != '\0';
    local_f8 = *(undefined8 *)(local_100 + 0x50);
    local_e8 = local_100 + 0x28;
    local_f0 = local_100 + 0x58;
    local_100 = local_100 + 0x48;
    local_e0 = 0;
    if ((*local_c8 & 0x88) == 0x88) {
      if (local_c8[1] == 0) {
        bVar3 = local_c8[2];
      }
      else {
        bVar3 = local_c8[2];
      }
      local_a0 = local_c8[1] != 0;
      local_9f = bVar3 != 0;
      if ((bool)local_9f) {
        bVar3 = local_c8[3];
      }
      else {
        bVar3 = local_c8[3];
      }
      local_9e = bVar3 != 0;
      local_b0 = local_c8 + 0x28;
      local_b8 = local_c8 + 0x58;
      local_c8 = local_c8 + 0x48;
      local_a8 = 0;
      if (*local_90 == -1) {
        if (local_90[1] == '\0') {
          cVar1 = local_90[2];
        }
        else {
          cVar1 = local_90[2];
        }
        local_68 = local_90[1] != '\0';
        if (cVar1 == '\0') {
          cVar2 = local_90[3];
        }
        else {
          cVar2 = local_90[3];
        }
        local_66 = cVar2 != '\0';
        local_67 = cVar1 != '\0';
        local_88 = *(undefined8 *)(local_90 + 0x50);
        local_78 = local_90 + 0x28;
        local_80 = local_90 + 0x58;
        local_90 = local_90 + 0x48;
        local_70 = 0;
        if ((*pbVar6 & 0x88) == 0x88) {
          if (pbVar6[1] == 0) {
            bVar3 = pbVar6[2];
          }
          else {
            bVar3 = pbVar6[2];
          }
          local_30 = pbVar6[1] != 0;
          if (bVar3 == 0) {
            bVar4 = pbVar6[3];
          }
          else {
            bVar4 = pbVar6[3];
          }
          local_2e = bVar4 != 0;
          local_2f = bVar3 != 0;
          local_50 = *(undefined8 *)(pbVar6 + 0x50);
          local_40 = pbVar6 + 0x28;
          local_48 = pbVar6 + 0x58;
          local_58 = pbVar6 + 0x48;
          local_38 = 0;
          local_10 = &local_152;
          local_20 = &local_148;
          local_28 = &DAT_ram_00033600;
          local_8 = 10;
          local_18 = 4;
          uVar5 = FUN_ram_0000cbc8(&local_28,&local_108,4,*(undefined8 *)(param_5 + -0xff0),
                                   *(undefined8 *)(param_5 + -0xfe8));
          return uVar5;
        }
        return 0xb;
      }
      return 0xb;
    }
    return 0xb;
  }
  if (*local_100 != -1) {
    return 0xb;
  }
  local_d8 = local_100[1] != '\0';
  local_d7 = local_100[2] != '\0';
  local_d6 = local_100[3] != '\0';
  local_f8 = *(undefined8 *)(local_100 + 0x50);
  local_e8 = local_100 + 0x28;
  local_f0 = local_100 + 0x58;
  local_100 = local_100 + 0x48;
  local_e0 = 0;
  if ((*local_c8 & 0x88) != 0x88) {
    return 0xb;
  }
  if (local_c8[1] == 0) {
    bVar3 = local_c8[2];
  }
  else {
    bVar3 = local_c8[2];
  }
  local_a0 = local_c8[1] != 0;
  local_9f = bVar3 != 0;
  if ((bool)local_9f) {
    bVar3 = local_c8[3];
  }
  else {
    bVar3 = local_c8[3];
  }
  local_9e = bVar3 != 0;
  local_b0 = local_c8 + 0x28;
  local_b8 = local_c8 + 0x58;
  local_c8 = local_c8 + 0x48;
  local_a8 = 0;
  if (*local_90 != -1) {
    return 0xb;
  }
  if (local_90[1] == '\0') {
    cVar1 = local_90[2];
  }
  else {
    cVar1 = local_90[2];
  }
  local_68 = local_90[1] != '\0';
  if (cVar1 == '\0') {
    cVar2 = local_90[3];
  }
  else {
    cVar2 = local_90[3];
  }
  local_66 = cVar2 != '\0';
  local_67 = cVar1 != '\0';
  local_88 = *(undefined8 *)(local_90 + 0x50);
  local_78 = local_90 + 0x28;
  local_80 = local_90 + 0x58;
  local_90 = local_90 + 0x48;
  local_70 = 0;
  if ((*pbVar6 & 0x88) == 0x88) {
    local_50 = *(undefined8 *)(pbVar6 + 0x50);
    local_40 = pbVar6 + 0x28;
    local_48 = pbVar6 + 0x58;
    local_58 = pbVar6 + 0x48;
    local_2e = pbVar6[3] != 0;
    local_2f = pbVar6[2] != 0;
    local_30 = pbVar6[1] != 0;
    local_38 = 0;
    local_10 = &local_152;
    local_20 = &local_148;
    local_28 = &DAT_ram_00033600;
    local_8 = 10;
    local_18 = 4;
    FUN_ram_0000cbc8(&local_28,&local_108,4,8,0);
    return 0x1a;
  }
  return 0xb;
}

// Function: FUN_ram_0000cbc8
undefined8 FUN_ram_0000cbc8(void)

{
  FUN_ram_0000cbc8();
  return 0x1a;
}

// Function: FUN_ram_0000cde0
void FUN_ram_0000cde0(undefined4 *param_1,longlong *param_2,undefined8 param_3,undefined8 param_4,
                     longlong param_5)

{
  bool bVar1;
  longlong lVar2;
  undefined4 local_28;
  undefined4 local_24;
  longlong *local_20;
  undefined8 local_18;
  undefined8 local_10;
  undefined8 local_8;
  
  lVar2 = *param_2;
  if ((((*(longlong *)(lVar2 + 0x28) != -0x6c5e9a281e0922fa) ||
       (*(longlong *)(lVar2 + 0x30) != -0x53861431b91e3427)) ||
      (*(longlong *)(lVar2 + 0x38) != -0x6ec8a4a0127a4be4)) ||
     (bVar1 = false, *(longlong *)(lVar2 + 0x40) != -0x56ff00817a0a73c6)) {
    bVar1 = true;
  }
  local_8 = *(undefined8 *)(param_5 + -0xff8);
  if (bVar1) {
    if (((*(longlong *)(lVar2 + 0x28) != -0x21708a111e0922fa) ||
        (*(longlong *)(lVar2 + 0x30) != -0x2532931b43a2bde8)) ||
       ((*(longlong *)(lVar2 + 0x38) != 0x270db9834dfc1ab6 ||
        (bVar1 = false, *(longlong *)(lVar2 + 0x40) != -0x3745e27d7064202)))) {
      bVar1 = true;
    }
    if (bVar1) {
      local_28 = 0;
      local_24 = 0xbadc0de3;
    }
    else {
      local_28 = FUN_ram_0000c220(param_2,param_3,param_4,*(undefined8 *)(param_5 + -0x1000));
      local_24 = 0xbadc0def;
    }
  }
  else {
    local_20 = param_2;
    local_18 = param_3;
    local_10 = param_4;
    FUN_ram_0002da18(&local_28,&local_20,8,0);
  }
  param_1[1] = local_24;
  *param_1 = local_28;
  return;
}

// Function: FUN_ram_0000d038
/* WARNING: Type propagation algorithm not settling */

void FUN_ram_0000d038(int *param_1,undefined8 *param_2,undefined8 *param_3,undefined8 *param_4,
                     longlong param_5)

{
  byte bVar1;
  char cVar2;
  bool bVar3;
  int iVar4;
  byte *pbVar5;
  undefined8 **ppuVar6;
  byte ****ppppbVar7;
  byte *pbVar8;
  undefined1 uVar9;
  undefined1 uVar10;
  undefined1 uVar11;
  int local_178;
  int iStack_174;
  undefined1 local_16a;
  undefined1 local_169;
  undefined8 local_168;
  byte ****local_160;
  undefined8 local_158;
  char *local_150;
  undefined2 local_148;
  byte *local_140;
  undefined2 local_138;
  undefined8 *local_130;
  undefined8 *local_128;
  undefined8 *local_120;
  undefined8 local_118;
  undefined8 local_110;
  undefined8 local_108;
  undefined8 local_100;
  undefined8 local_f8;
  undefined1 *local_f0;
  undefined8 local_e8;
  undefined8 **local_e0;
  undefined8 local_d8;
  byte ****local_d0;
  byte *local_c8;
  ulonglong local_c0;
  byte *local_b8;
  byte *local_b0;
  undefined8 local_a8;
  undefined8 local_a0;
  char *local_98;
  char *local_90;
  ulonglong local_88;
  char *local_80;
  char *local_78;
  undefined8 local_70;
  undefined1 local_68;
  undefined1 local_67;
  undefined1 local_66;
  byte *local_60;
  byte *local_58;
  undefined8 local_50;
  byte *local_48;
  byte *local_40;
  undefined8 local_38;
  undefined1 local_30;
  undefined1 local_2f;
  undefined1 local_2e;
  undefined *local_28;
  byte *****local_20;
  undefined8 local_18;
  undefined1 *local_10;
  undefined8 local_8;
  
  local_16a = (undefined1)*(undefined8 *)(param_5 + -0xfe8);
  pbVar5 = (byte *)*param_2;
  local_b0 = pbVar5 + 0x28;
  if ((((*(longlong *)(pbVar5 + 0x28) != -0x6c5e9a281e0922fa) ||
       (*(longlong *)(pbVar5 + 0x30) != -0x53861431b91e3427)) ||
      (*(longlong *)(pbVar5 + 0x38) != -0x6ec8a4a0127a4be4)) ||
     (bVar3 = false, *(longlong *)(pbVar5 + 0x40) != -0x56ff00817a0a73c6)) {
    bVar3 = true;
  }
  local_168 = *(undefined8 *)(param_5 + -0xfe0);
  local_100 = *(undefined8 *)(param_5 + -0xff0);
  local_110 = *(undefined8 *)(param_5 + -0xff8);
  local_c0 = *(undefined8 *)(param_5 + -0x1000);
  if (bVar3) {
    if (((*(longlong *)local_b0 != -0x21708a111e0922fa) ||
        (*(longlong *)(pbVar5 + 0x30) != -0x2532931b43a2bde8)) ||
       ((*(longlong *)(pbVar5 + 0x38) != 0x270db9834dfc1ab6 ||
        (bVar3 = false, *(longlong *)(pbVar5 + 0x40) != -0x3745e27d7064202)))) {
      bVar3 = true;
    }
    if (bVar3) {
      local_178 = 0;
      iStack_174 = -0x4523f21d;
      goto LAB_ram_0000d778;
    }
    local_169 = 3;
    local_90 = (char *)*param_3;
    pbVar8 = (byte *)*param_4;
    local_160 = (byte ****)(pbVar5 + 8);
    local_140 = pbVar8 + 8;
    local_150 = local_90 + 8;
    local_138 = 0x100;
    local_148 = 1;
    local_158 = CONCAT62(local_158._2_6_,1);
    local_f0 = &local_16a;
    local_130 = (undefined8 *)&DAT_ram_00033bb2;
    local_e8 = 1;
    local_f8 = 0x20;
    local_108 = 0x20;
    local_118 = 0x20;
    local_128 = (undefined8 *)0x6;
    local_e0 = &local_130;
    local_d8 = 5;
    local_88 = (ulonglong)*pbVar5;
    if ((ulonglong)*pbVar5 == 0xff) {
      uVar9 = 1;
      if (pbVar5[1] == 0) {
        uVar9 = 0;
        if (pbVar5[2] == 0) goto LAB_ram_0000d7b8;
LAB_ram_0000d470:
        uVar11 = 1;
        uVar10 = 1;
        bVar1 = pbVar5[3];
      }
      else {
        if (pbVar5[2] != 0) goto LAB_ram_0000d470;
LAB_ram_0000d7b8:
        uVar11 = 0;
        uVar10 = 0;
        bVar1 = pbVar5[3];
      }
      if (bVar1 == 0) {
        uVar10 = uVar11;
      }
      local_b8 = pbVar5 + 0x58;
      local_c8 = pbVar5 + 0x48;
      local_a0 = CONCAT71(CONCAT61(CONCAT51(local_a0._3_5_,bVar1 != 0),uVar10),uVar9);
      local_a8 = 0;
      local_88 = *(ulonglong *)(pbVar5 + 0x50);
      if (*local_90 == -1) {
        local_68 = 1;
        if (local_90[1] == '\0') {
          local_68 = 0;
          if (local_90[2] == '\0') goto LAB_ram_0000d7f8;
LAB_ram_0000d538:
          uVar9 = 1;
          local_67 = 1;
          cVar2 = local_90[3];
        }
        else {
          if (local_90[2] != '\0') goto LAB_ram_0000d538;
LAB_ram_0000d7f8:
          uVar9 = 0;
          local_67 = 0;
          cVar2 = local_90[3];
        }
        if (cVar2 == '\0') {
          local_67 = uVar9;
        }
        local_66 = cVar2 != '\0';
        local_88 = *(ulonglong *)(local_90 + 0x50);
        local_78 = local_90 + 0x28;
        local_80 = local_90 + 0x58;
        local_90 = local_90 + 0x48;
        local_70 = 0;
        if ((*pbVar8 & 0x88) == 0x88) {
          local_50 = *(undefined8 *)(pbVar8 + 0x50);
          local_40 = pbVar8 + 0x28;
          local_48 = pbVar8 + 0x58;
          local_58 = pbVar8 + 0x48;
          if (pbVar8[3] == 0) {
            local_2e = 0;
            if (pbVar8[2] == 0) goto LAB_ram_0000d838;
LAB_ram_0000d688:
            local_2f = 1;
          }
          else {
            local_2e = 1;
            if (pbVar8[2] != 0) goto LAB_ram_0000d688;
LAB_ram_0000d838:
            local_2f = 0;
          }
          local_30 = pbVar8[1] != 0;
          local_38 = 0;
          local_10 = &local_169;
          local_20 = &local_160;
          local_28 = &DAT_ram_00033600;
          local_8 = 9;
          local_18 = 3;
          ppppbVar7 = (byte ****)&local_d0;
          local_120 = (undefined8 *)local_c0;
          local_d0 = local_160;
          local_c0 = *(ulonglong *)(pbVar5 + 0x50);
          local_98 = local_150;
          local_60 = local_140;
          FUN_ram_0000d750(&local_28,ppppbVar7,3,&local_e0,1);
          iVar4 = (int)ppppbVar7;
          goto LAB_ram_0000d2f8;
        }
      }
    }
    iStack_174 = (int)local_88;
    local_178 = 0xb;
  }
  else {
    local_90 = &local_16a;
    local_d0 = (byte ****)&DAT_ram_00033bb2;
    local_88 = 1;
    local_98 = (char *)0x20;
    local_a8 = 0x20;
    local_b8 = (byte *)0x20;
    local_c8 = (byte *)0x6;
    local_160 = (byte ****)&local_d0;
    local_158 = 5;
    ppuVar6 = &local_130;
    local_130 = param_2;
    local_128 = param_3;
    local_120 = param_4;
    local_118 = local_168;
    local_b0 = (byte *)local_110;
    local_a0 = local_100;
    FUN_ram_0002da18(&local_178,ppuVar6,&local_160,1);
    iVar4 = (int)ppuVar6;
    if (local_178 != 0x1a) goto LAB_ram_0000d778;
LAB_ram_0000d2f8:
    iStack_174 = iVar4;
    local_178 = 0x1a;
  }
LAB_ram_0000d778:
  param_1[1] = iStack_174;
  *param_1 = local_178;
  return;
}

// Function: FUN_ram_0000d750
void FUN_ram_0000d750(undefined8 param_1,undefined4 param_2)

{
  undefined4 *puStack_180;
  
  FUN_ram_0000d750();
  puStack_180[1] = param_2;
  *puStack_180 = 0x1a;
  return;
}

// Function: FUN_ram_0000d860
void FUN_ram_0000d860(int *param_1,undefined **param_2,undefined8 param_3,undefined8 param_4,
                     longlong param_5)

{
  bool bVar1;
  undefined *puVar2;
  int *piVar3;
  int local_90;
  int local_8c;
  undefined1 local_81;
  undefined **local_80;
  undefined8 local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined **local_60;
  undefined8 local_58;
  undefined *local_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_18;
  undefined ***local_10;
  undefined8 local_8;
  
  local_68 = *(undefined8 *)(param_5 + -0xfd8);
  local_81 = (undefined1)*(undefined8 *)(param_5 + -0xfe0);
  puVar2 = *param_2;
  if ((((*(longlong *)(puVar2 + 0x28) != -0x6c5e9a281e0922fa) ||
       (*(longlong *)(puVar2 + 0x30) != -0x53861431b91e3427)) ||
      (*(longlong *)(puVar2 + 0x38) != -0x6ec8a4a0127a4be4)) ||
     (bVar1 = false, *(longlong *)(puVar2 + 0x40) != -0x56ff00817a0a73c6)) {
    bVar1 = true;
  }
  local_20 = *(undefined8 *)(param_5 + -0xfe8);
  local_30 = *(undefined8 *)(param_5 + -0xff0);
  local_40 = *(undefined8 *)(param_5 + -0xff8);
  if (bVar1) {
    if (((*(longlong *)(puVar2 + 0x28) != -0x21708a111e0922fa) ||
        (*(longlong *)(puVar2 + 0x30) != -0x2532931b43a2bde8)) ||
       ((*(longlong *)(puVar2 + 0x38) != 0x270db9834dfc1ab6 ||
        (bVar1 = false, *(longlong *)(puVar2 + 0x40) != -0x3745e27d7064202)))) {
      bVar1 = true;
    }
    if (bVar1) {
      local_90 = 0;
      local_8c = -0x4523f21d;
    }
    else {
      local_60 = (undefined **)CONCAT71(local_60._1_7_,local_81);
      local_10 = &local_60;
      local_50 = &DAT_ram_00033bb2;
      local_8 = 1;
      local_18 = 0x20;
      local_28 = 0x20;
      local_38 = 0x20;
      local_48 = 6;
      local_80 = &local_50;
      local_78 = 5;
      local_90 = FUN_ram_0000c220(param_2,param_3,param_4,*(undefined8 *)(param_5 + -0x1000));
      local_8c = -0x4523f211;
    }
  }
  else {
    local_10 = (undefined ***)&local_81;
    local_50 = &DAT_ram_00033bb2;
    local_8 = 1;
    local_18 = 0x20;
    local_28 = 0x20;
    local_38 = 0x20;
    local_48 = 6;
    local_60 = &local_50;
    local_58 = 5;
    piVar3 = &local_90;
    local_80 = param_2;
    local_78 = param_3;
    local_70 = param_4;
    FUN_ram_0002da18(piVar3,&local_80,&local_60,1);
    if (local_90 == 0x1a) {
      local_90 = 0x1a;
      local_8c = (int)piVar3;
    }
  }
  param_1[1] = local_8c;
  *param_1 = local_90;
  return;
}

// Function: FUN_ram_0000dc68
/* WARNING: Removing unreachable block (ram,0x0000e490) */

void FUN_ram_0000dc68(undefined4 *param_1,longlong *param_2,ulonglong param_3,longlong param_4,
                     longlong param_5)

{
  bool bVar1;
  bool bVar2;
  longlong *plVar3;
  undefined1 uVar4;
  longlong lVar5;
  longlong lVar6;
  undefined8 uVar7;
  undefined1 uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  undefined8 uVar13;
  int iVar14;
  undefined8 uVar15;
  longlong *plVar16;
  ulonglong uVar17;
  undefined8 uVar18;
  longlong *plVar19;
  undefined8 uVar20;
  ulonglong uVar21;
  ulonglong uVar22;
  longlong *plVar23;
  int iVar24;
  longlong *plVar25;
  longlong *plVar26;
  ulonglong uVar27;
  longlong lVar28;
  longlong lVar29;
  ulonglong uVar30;
  longlong lVar31;
  longlong *plVar32;
  ulonglong uVar33;
  undefined8 uVar34;
  longlong lVar35;
  longlong *plVar36;
  longlong lVar37;
  longlong lVar38;
  ulonglong uVar39;
  ulonglong uVar40;
  undefined4 uVar41;
  undefined8 *puVar42;
  ulonglong uVar43;
  undefined8 uVar44;
  longlong *local_238;
  longlong *local_230;
  ulonglong local_218;
  longlong *local_210;
  longlong *local_1e8;
  longlong local_1d0;
  undefined1 local_1b8;
  uint uStack_198;
  uint uStack_194;
  uint uStack_190;
  uint uStack_18c;
  uint uStack_188;
  uint uStack_184;
  ulonglong uStack_180;
  ulonglong uStack_178;
  ulonglong uStack_170;
  ulonglong uStack_168;
  longlong **pplStack_160;
  ulonglong uStack_158;
  ulonglong uStack_150;
  ulonglong uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  longlong lStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  longlong *local_100;
  longlong *local_f8;
  longlong *local_f0;
  ulonglong local_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  longlong lStack_c8;
  ulonglong uStack_c0;
  ulonglong uStack_b8;
  ulonglong uStack_b0;
  ulonglong uStack_a8;
  ulonglong uStack_a0;
  ulonglong uStack_98;
  ulonglong uStack_90;
  ulonglong uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  ulonglong uStack_70;
  ulonglong uStack_68;
  ulonglong uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  longlong lStack_30;
  longlong lStack_28;
  ulonglong uStack_20;
  ulonglong uStack_18;
  ulonglong uStack_10;
  undefined1 uStack_8;
  undefined1 uStack_7;
  undefined1 uStack_6;
  undefined1 uStack_5;
  
  uVar41 = 0xbadc0de;
  uVar30 = *(ulonglong *)(param_5 + -0xfe0);
  lVar31 = *(longlong *)(param_5 + -0xfe8);
  uVar34 = *(undefined8 *)(param_5 + -0xff0);
  lVar35 = *(longlong *)(param_5 + -0x1000);
  plVar36 = *(longlong **)(param_5 + -0xfd8);
  lVar37 = *plVar36;
  plVar25 = *(longlong **)(param_5 + -0xff8);
  uVar9 = 0;
  if ((plVar25 == (longlong *)0x0) && (lVar37 != 0)) goto LAB_ram_0000e970;
  if ((*(longlong *)(param_4 + 8) != -0x368b38e7e82a58fa) ||
     (((*(longlong *)(param_4 + 0x10) != -0x49a1e296679ca9d8 ||
       (*(longlong *)(param_4 + 0x18) != 0x5c6d4b9ba3b85e8b)) ||
      (bVar1 = false, *(longlong *)(param_4 + 0x20) != 0x215b5573)))) {
    bVar1 = true;
  }
  uVar41 = 0xabad1dea;
  if (bVar1) goto LAB_ram_0000e970;
  plVar26 = param_2 + 1;
  lVar38 = *plVar26;
  if (((*(longlong *)(lVar38 + 0x28) != -0x16a608d8d48b0286) ||
      (*(longlong *)(lVar38 + 0x30) != 0x7a819dd33c7070c6)) ||
     ((*(longlong *)(lVar38 + 0x38) != 0x6dd2523bce0a93a0 ||
      (bVar1 = false, *(longlong *)(lVar38 + 0x40) != -0x2c4478dc22ab5fac)))) {
    bVar1 = true;
  }
  if (bVar1) {
    uVar9 = 0;
    goto LAB_ram_0000e970;
  }
  if (((*(longlong *)(lVar35 + 8) != 0x66d17b1817d5a706) ||
      (*(longlong *)(lVar35 + 0x10) != -0x3f3d02aafb2b25cb)) ||
     ((*(longlong *)(lVar35 + 0x18) != -0x5a8aa9de7039db3f ||
      (bVar1 = false, *(longlong *)(lVar35 + 0x20) != 0x85fcbbadb)))) {
    bVar1 = true;
  }
  uVar9 = 0;
  if (bVar1) goto LAB_ram_0000e970;
  if (*(longlong *)(lVar38 + 0x50) != 0x6c0) {
                    /* WARNING: Subroutine does not return */
    FUN_ram_000011b0(&DAT_ram_000337e9,0xe,2);
  }
  if ((lVar38 + 0x58U & 7) != 0) {
                    /* WARNING: Subroutine does not return */
    FUN_ram_000011b0(&DAT_ram_000337e9,0xe,0);
  }
  uVar41 = 0xbadc0de2;
  uVar27 = *(ulonglong *)(lVar38 + 0x288) ^ 0xffffffffffffffa1;
  if (2 < (uVar27 & 0xff)) goto LAB_ram_0000e970;
  if (*(ulonglong *)(param_5 + -0xfd0) < param_3) {
    plVar16 = param_2 + *(ulonglong *)(param_5 + -0xfd0);
  }
  else {
    plVar16 = (longlong *)0x0;
  }
  uVar43 = *(ulonglong *)(param_4 + 0x58);
  uVar39 = *(ulonglong *)(lVar38 + 0x2b0);
  lVar35 = *param_2;
  puVar42 = (undefined8 *)(lVar35 + 8);
  FUN_ram_00007978(&local_100,uVar27,plVar16,lVar38 + 0x58U,puVar42);
  if ((int)local_100 != 0) {
LAB_ram_0000e078:
    uVar27 = (ulonglong)local_f8 & 0xffffffff;
    uVar9 = (ulonglong)local_100 >> 0x20;
    goto LAB_ram_0000e088;
  }
  local_1e8 = param_2 + 3;
  plVar16 = param_2 + 2;
  uVar9 = *(ulonglong *)(lVar38 + 0x248) ^ 0x4a1178751b9c3c6;
  uVar27 = *(ulonglong *)(lVar38 + 0x250) ^ 0x4a0178651b8c3c5;
  uVar21 = *(ulonglong *)(lVar38 + 0x240) ^ 0x4a2178451bac3c7;
  uVar17 = *(ulonglong *)(lVar38 + 0x238) ^ 0xfb5ce87aae443c38;
  uVar17 = uVar17 << 0x38 | (uVar17 & 0xff00) << 0x28 | (uVar17 & 0xff0000) << 0x18 |
           (uVar17 & 0xff000000) << 8 | uVar17 >> 8 & 0xff000000 | uVar17 >> 0x18 & 0xff0000 |
           uVar17 >> 0x28 & 0xff00 | uVar17 >> 0x38;
  lVar28 = *plVar16;
  uVar22 = *(ulonglong *)(lVar28 + 8);
  uVar22 = uVar22 << 0x38 | (uVar22 & 0xff00) << 0x28 | (uVar22 & 0xff0000) << 0x18 |
           (uVar22 & 0xff000000) << 8 | uVar22 >> 8 & 0xff000000 | uVar22 >> 0x18 & 0xff0000 |
           uVar22 >> 0x28 & 0xff00 | uVar22 >> 0x38;
  if (uVar17 == uVar22) {
    uVar17 = uVar21 << 0x38 | (uVar21 & 0xff00) << 0x28 | (uVar21 & 0xff0000) << 0x18 |
             (uVar21 & 0xff000000) << 8 | uVar21 >> 8 & 0xff000000 | uVar21 >> 0x18 & 0xff0000 |
             uVar21 >> 0x28 & 0xff00 | uVar21 >> 0x38;
    uVar21 = *(ulonglong *)(lVar28 + 0x10);
    uVar22 = uVar21 << 0x38 | (uVar21 & 0xff00) << 0x28 | (uVar21 & 0xff0000) << 0x18 |
             (uVar21 & 0xff000000) << 8 | uVar21 >> 8 & 0xff000000 | uVar21 >> 0x18 & 0xff0000 |
             uVar21 >> 0x28 & 0xff00 | uVar21 >> 0x38;
    if (uVar17 != uVar22) goto LAB_ram_0000e230;
    uVar17 = uVar9 << 0x38 | (uVar9 & 0xff00) << 0x28 | (uVar9 & 0xff0000) << 0x18 |
             (uVar9 & 0xff000000) << 8 | uVar9 >> 8 & 0xff000000 | uVar9 >> 0x18 & 0xff0000 |
             uVar9 >> 0x28 & 0xff00 | uVar9 >> 0x38;
    uVar9 = *(ulonglong *)(lVar28 + 0x18);
    uVar22 = uVar9 << 0x38 | (uVar9 & 0xff00) << 0x28 | (uVar9 & 0xff0000) << 0x18 |
             (uVar9 & 0xff000000) << 8 | uVar9 >> 8 & 0xff000000 | uVar9 >> 0x18 & 0xff0000 |
             uVar9 >> 0x28 & 0xff00 | uVar9 >> 0x38;
    if (uVar17 != uVar22) goto LAB_ram_0000e230;
    iVar14 = 0;
    uVar17 = uVar27 << 0x38 | (uVar27 & 0xff00) << 0x28 | (uVar27 & 0xff0000) << 0x18 |
             (uVar27 & 0xff000000) << 8 | uVar27 >> 8 & 0xff000000 | uVar27 >> 0x18 & 0xff0000 |
             uVar27 >> 0x28 & 0xff00 | uVar27 >> 0x38;
    uVar9 = *(ulonglong *)(lVar28 + 0x20);
    uVar22 = uVar9 << 0x38 | (uVar9 & 0xff00) << 0x28 | (uVar9 & 0xff0000) << 0x18 |
             (uVar9 & 0xff000000) << 8 | uVar9 >> 8 & 0xff000000 | uVar9 >> 0x18 & 0xff0000 |
             uVar9 >> 0x28 & 0xff00 | uVar9 >> 0x38;
    if (uVar17 != uVar22) goto LAB_ram_0000e230;
  }
  else {
LAB_ram_0000e230:
    iVar14 = -1;
    if (uVar22 <= uVar17) {
      iVar14 = 1;
    }
  }
  local_f0 = (longlong *)(*(ulonglong *)(lVar38 + 0x228) ^ 0x4a1178751b9c3c6);
  local_e8 = *(ulonglong *)(lVar38 + 0x230) ^ 0x4a0178651b8c3c5;
  local_f8 = (longlong *)(*(ulonglong *)(lVar38 + 0x220) ^ 0x4a2178451bac3c7);
  local_100 = (longlong *)(*(ulonglong *)(lVar38 + 0x218) ^ 0xfb5ce87aae443c38);
  uVar9 = (longlong)local_100 << 0x38 | ((ulonglong)local_100 & 0xff00) << 0x28 |
          ((ulonglong)local_100 & 0xff0000) << 0x18 | ((ulonglong)local_100 & 0xff000000) << 8 |
          (ulonglong)local_100 >> 8 & 0xff000000 | (ulonglong)local_100 >> 0x18 & 0xff0000 |
          (ulonglong)local_100 >> 0x28 & 0xff00 | (ulonglong)local_100 >> 0x38;
  lVar5 = *local_1e8;
  uVar27 = *(ulonglong *)(lVar5 + 8);
  uVar27 = uVar27 << 0x38 | (uVar27 & 0xff00) << 0x28 | (uVar27 & 0xff0000) << 0x18 |
           (uVar27 & 0xff000000) << 8 | uVar27 >> 8 & 0xff000000 | uVar27 >> 0x18 & 0xff0000 |
           uVar27 >> 0x28 & 0xff00 | uVar27 >> 0x38;
  if (uVar9 == uVar27) {
    uVar9 = (longlong)local_f8 << 0x38 | ((ulonglong)local_f8 & 0xff00) << 0x28 |
            ((ulonglong)local_f8 & 0xff0000) << 0x18 | ((ulonglong)local_f8 & 0xff000000) << 8 |
            (ulonglong)local_f8 >> 8 & 0xff000000 | (ulonglong)local_f8 >> 0x18 & 0xff0000 |
            (ulonglong)local_f8 >> 0x28 & 0xff00 | (ulonglong)local_f8 >> 0x38;
    uVar27 = *(ulonglong *)(lVar5 + 0x10);
    uVar27 = uVar27 << 0x38 | (uVar27 & 0xff00) << 0x28 | (uVar27 & 0xff0000) << 0x18 |
             (uVar27 & 0xff000000) << 8 | uVar27 >> 8 & 0xff000000 | uVar27 >> 0x18 & 0xff0000 |
             uVar27 >> 0x28 & 0xff00 | uVar27 >> 0x38;
    if (uVar9 != uVar27) goto LAB_ram_0000e398;
    uVar9 = (longlong)local_f0 << 0x38 | ((ulonglong)local_f0 & 0xff00) << 0x28 |
            ((ulonglong)local_f0 & 0xff0000) << 0x18 | ((ulonglong)local_f0 & 0xff000000) << 8 |
            (ulonglong)local_f0 >> 8 & 0xff000000 | (ulonglong)local_f0 >> 0x18 & 0xff0000 |
            (ulonglong)local_f0 >> 0x28 & 0xff00 | (ulonglong)local_f0 >> 0x38;
    uVar27 = *(ulonglong *)(lVar5 + 0x18);
    uVar27 = uVar27 << 0x38 | (uVar27 & 0xff00) << 0x28 | (uVar27 & 0xff0000) << 0x18 |
             (uVar27 & 0xff000000) << 8 | uVar27 >> 8 & 0xff000000 | uVar27 >> 0x18 & 0xff0000 |
             uVar27 >> 0x28 & 0xff00 | uVar27 >> 0x38;
    if (uVar9 != uVar27) goto LAB_ram_0000e398;
    iVar24 = 0;
    uVar9 = local_e8 << 0x38 | (local_e8 & 0xff00) << 0x28 | (local_e8 & 0xff0000) << 0x18 |
            (local_e8 & 0xff000000) << 8 | local_e8 >> 8 & 0xff000000 | local_e8 >> 0x18 & 0xff0000
            | local_e8 >> 0x28 & 0xff00 | local_e8 >> 0x38;
    uVar27 = *(ulonglong *)(lVar5 + 0x20);
    uVar27 = uVar27 << 0x38 | (uVar27 & 0xff00) << 0x28 | (uVar27 & 0xff0000) << 0x18 |
             (uVar27 & 0xff000000) << 8 | uVar27 >> 8 & 0xff000000 | uVar27 >> 0x18 & 0xff0000 |
             uVar27 >> 0x28 & 0xff00 | uVar27 >> 0x38;
    if (uVar9 != uVar27) goto LAB_ram_0000e398;
  }
  else {
LAB_ram_0000e398:
    iVar24 = -1;
    if (uVar27 <= uVar9) {
      iVar24 = 1;
    }
  }
  if (iVar24 != 0 || iVar14 != 0) {
    uVar41 = 0xbadface3;
    uVar9 = 0;
    goto LAB_ram_0000e970;
  }
  plVar23 = (longlong *)(*(ulonglong *)(lVar38 + 0x2c0) ^ 0x6e9de2b30b19f1ea);
  uVar27 = uVar43 - (longlong)plVar23;
  uVar9 = 0;
  if ((uVar27 <= uVar43) &&
     (uVar41 = 0xfaded, (*(ulonglong *)(lVar38 + 0x2b8) ^ 0x6e9de2b30b19f1ea) < uVar27))
  goto LAB_ram_0000e970;
  uVar41 = 0xfaded;
  lVar6 = FUN_ram_0000e500(&local_100);
  if (lVar6 != 0) {
    uVar30 = lVar6 << 0x20 | lVar6 - 0x100000000U >> 0x20;
    if (uVar30 < 0x1a) {
      uVar41 = *(undefined4 *)(&DAT_ram_00033d50 + uVar30 * 4);
      uVar9 = uVar30;
    }
    else {
      uVar41 = (undefined4)lVar6;
    }
    goto LAB_ram_0000e970;
  }
  if (plVar23 <= local_100) goto LAB_ram_0000e970;
  plVar32 = (longlong *)(lVar38 + 8);
  uVar4 = FUN_ram_00012848(uVar34,puVar42,plVar32,plVar23);
  uVar27 = uVar30 & 0xff;
  bVar1 = false;
  if (uVar27 == 0) {
LAB_ram_0000e658:
    uVar27 = 0xb1ade2;
    uVar41 = 0xb1ade2;
    if (0x47 < *(ulonglong *)(lVar28 + 0x50)) {
      if (*(ulonglong *)(lVar5 + 0x50) < 0x48) goto LAB_ram_0000e970;
      uVar34 = *(undefined8 *)(lVar28 + 0x98);
      uVar18 = *(undefined8 *)(lVar5 + 0x98);
      uStack_148 = *(longlong *)(lVar38 + 0x20);
      uStack_150 = *(longlong *)(lVar38 + 0x18);
      uStack_158 = *(longlong *)(lVar38 + 0x10);
      pplStack_160 = (longlong **)*plVar32;
      FUN_ram_00026e90(&local_100,lVar31,uVar18,uVar34);
      uVar43 = local_e8;
      plVar3 = local_f0;
      plVar23 = local_f8;
      if ((int)local_100 == 1) goto LAB_ram_0000e078;
      local_210 = param_2 + 5;
      plVar19 = param_2 + 4;
      if (bVar1) {
        local_238 = local_1e8;
        local_230 = local_210;
        local_210 = plVar19;
        local_1e8 = plVar16;
        if (plVar25 != (longlong *)0x0) {
          lVar29 = 0x210;
          lVar28 = 0x208;
          lVar5 = 0x200;
          lVar6 = 0x1f8;
LAB_ram_0000ea48:
          uStack_170 = *(ulonglong *)(lVar38 + lVar28) ^ 0x4a1178751b9c3c6;
          uStack_168 = *(ulonglong *)(lVar38 + lVar29) ^ 0x4a0178651b8c3c5;
          uStack_178 = *(ulonglong *)(lVar38 + lVar5) ^ 0x4a2178451bac3c7;
          uStack_180 = *(ulonglong *)(lVar38 + lVar6) ^ 0xfb5ce87aae443c38;
          if (*(ulonglong *)(lVar38 + 0x710) < 5) {
            local_f0 = (longlong *)(*(ulonglong *)(lVar38 + 0x268) ^ 0x4a1178751b9c3c6);
            local_e8 = *(ulonglong *)(lVar38 + 0x270) ^ 0x4a0178651b8c3c5;
            local_f8 = (longlong *)(*(ulonglong *)(lVar38 + 0x260) ^ 0x4a2178451bac3c7);
            local_100 = (longlong *)(*(ulonglong *)(lVar38 + 600) ^ 0xfb5ce87aae443c38);
          }
          else {
            local_e8 = *(ulonglong *)(lVar38 + 0x270);
            local_f0 = *(longlong **)(lVar38 + 0x268);
            local_f8 = *(longlong **)(lVar38 + 0x260);
            local_100 = *(longlong **)(lVar38 + 600);
          }
          FUN_ram_0002a740(&pplStack_160,&local_100,&uStack_180);
          lVar28 = *plVar25;
          if ((((*(longlong ***)(lVar28 + 8) != pplStack_160) ||
               (*(longlong *)(lVar28 + 0x10) != uStack_158)) ||
              (*(longlong *)(lVar28 + 0x18) != uStack_150)) ||
             (bVar2 = false, *(longlong *)(lVar28 + 0x20) != uStack_148)) {
            bVar2 = true;
          }
          if (bVar2) {
            uVar27 = 0xbadc0de8;
            goto LAB_ram_0000e088;
          }
        }
      }
      else {
        local_238 = plVar16;
        local_230 = plVar19;
        if (plVar25 != (longlong *)0x0) {
          lVar29 = 0x1f0;
          lVar28 = 0x1e8;
          lVar5 = 0x1e0;
          lVar6 = 0x1d8;
          goto LAB_ram_0000ea48;
        }
      }
      local_218 = 0;
      if (lVar37 != 0) {
        local_218 = (ulonglong)(plVar36[1] * lStack_c8) / 10000;
      }
      if (*(ulonglong *)(lVar38 + 0x710) < 5) {
        uStack_170 = *(ulonglong *)(lVar38 + 0x268) ^ 0x4a1178751b9c3c6;
        uStack_168 = *(ulonglong *)(lVar38 + 0x270) ^ 0x4a0178651b8c3c5;
        uStack_178 = *(ulonglong *)(lVar38 + 0x260) ^ 0x4a2178451bac3c7;
        uStack_180 = *(ulonglong *)(lVar38 + 600) ^ 0xfb5ce87aae443c38;
      }
      else {
        uStack_168 = *(ulonglong *)(lVar38 + 0x270);
        uStack_170 = *(ulonglong *)(lVar38 + 0x268);
        uStack_178 = *(ulonglong *)(lVar38 + 0x260);
        uStack_180 = *(ulonglong *)(lVar38 + 600);
      }
      uStack_148 = *(ulonglong *)(lVar38 + 0x210) ^ 0x4a0178651b8c3c5;
      uStack_150 = *(ulonglong *)(lVar38 + 0x208) ^ 0x4a1178751b9c3c6;
      uStack_158 = *(ulonglong *)(lVar38 + 0x200) ^ 0x4a2178451bac3c7;
      pplStack_160 = (longlong **)(*(ulonglong *)(lVar38 + 0x1f8) ^ 0xfb5ce87aae443c38);
      local_e8 = *(ulonglong *)(lVar38 + 0x1f0) ^ 0x4a0178651b8c3c5;
      local_f0 = (longlong *)(*(ulonglong *)(lVar38 + 0x1e8) ^ 0x4a1178751b9c3c6);
      local_f8 = (longlong *)(*(ulonglong *)(lVar38 + 0x1e0) ^ 0x4a2178451bac3c7);
      local_100 = (longlong *)(*(ulonglong *)(lVar38 + 0x1d8) ^ 0xfb5ce87aae443c38);
      FUN_ram_0000d038(&uStack_188,local_1e8,local_210,plVar26);
      uVar9 = (ulonglong)uStack_188;
      if (uVar9 == 0x1a) {
        if (local_218 == 0) {
LAB_ram_0000f390:
          local_f8 = local_238;
          local_100 = local_230;
          local_f0 = param_2;
          local_e8 = lVar31;
          FUN_ram_0002da18(&uStack_198,&local_100,8,0);
          uVar9 = (ulonglong)uStack_198;
          if (uVar9 == 0x1a) {
            local_1d0 = lVar31;
            lVar28 = lStack_c8;
            if (!bVar1) {
              local_1d0 = lStack_c8;
              lVar28 = lVar31;
            }
            uStack_128 = *(undefined8 *)(lVar35 + 0x20);
            uStack_130 = *(undefined8 *)(lVar35 + 0x18);
            uStack_138 = *(undefined8 *)(lVar35 + 0x10);
            uStack_140 = *puVar42;
            lStack_120 = *plVar32;
            uStack_118 = *(undefined8 *)(lVar38 + 0x10);
            uStack_110 = *(undefined8 *)(lVar38 + 0x18);
            uStack_108 = *(undefined8 *)(lVar38 + 0x20);
            lVar31 = plVar36[3];
            local_1b8 = (undefined1)plVar36[1];
            uVar9 = *(ulonglong *)(lVar38 + 0x210);
            uVar17 = *(ulonglong *)(lVar38 + 0x1f0);
            uVar21 = *(ulonglong *)(lVar38 + 0x208);
            uVar22 = *(ulonglong *)(lVar38 + 0x1e8);
            uVar10 = *(ulonglong *)(lVar38 + 0x200);
            uVar11 = *(ulonglong *)(lVar38 + 0x1e0);
            uVar33 = *(ulonglong *)(lVar38 + 0x1f8);
            uVar12 = *(ulonglong *)(lVar38 + 0x1d8);
            uVar40 = *(ulonglong *)(lVar38 + 0x378);
            uVar27 = *(ulonglong *)(lVar38 + 800);
            uVar7 = *(undefined8 *)(lVar38 + 0x298);
            uVar44 = *(undefined8 *)(lVar38 + 0x2a0);
            uVar13 = *puVar42;
            uVar15 = *(undefined8 *)(lVar35 + 0x10);
            uVar20 = *(undefined8 *)(lVar35 + 0x18);
            *(undefined8 *)(lVar38 + 0x2e0) = *(undefined8 *)(lVar35 + 0x20);
            *(undefined8 *)(lVar38 + 0x2d8) = uVar20;
            *(undefined8 *)(lVar38 + 0x2d0) = uVar15;
            *(undefined8 *)(lVar38 + 0x2c8) = uVar13;
            *(undefined8 *)(lVar38 + 0x2f0) = uVar44;
            *(undefined8 *)(lVar38 + 0x2e8) = uVar7;
            uVar8 = (undefined1)uVar30;
            *(undefined1 *)(lVar38 + 0x308) = uVar8;
            *(longlong *)(lVar38 + 0x300) = local_1d0;
            *(longlong *)(lVar38 + 0x2f8) = lVar28;
            *(undefined8 *)(lVar38 + 0x309) = 0;
            *(undefined8 *)(lVar38 + 0x310) = 0;
            FUN_ram_00002720(lVar38 + 0x2c8);
            FUN_ram_00031b28(&local_100,&uStack_140,0x40);
            if (lVar37 == 0) {
              local_1b8 = 0;
            }
            uStack_20 = uVar39 ^ 0x6e9de2b30b19f9ea;
            uStack_5 = (char)lVar31;
            if (lVar37 == 0) {
              uStack_5 = 0;
            }
            uVar27 = uVar27 ^ 0xd3198133b7c1776c;
            uStack_10 = uVar40 ^ 0x504156a22548f8dd;
            uStack_c0 = uVar12 ^ 0xfb5ce87aae443c38;
            uStack_a0 = uVar33 ^ 0xfb5ce87aae443c38;
            uStack_b8 = uVar11 ^ 0x4a2178451bac3c7;
            uStack_98 = uVar10 ^ 0x4a2178451bac3c7;
            uStack_b0 = uVar22 ^ 0x4a1178751b9c3c6;
            uStack_90 = uVar21 ^ 0x4a1178751b9c3c6;
            uStack_a8 = uVar17 ^ 0x4a0178651b8c3c5;
            uStack_88 = uVar9 ^ 0x4a0178651b8c3c5;
            uStack_48 = uStack_d0;
            uStack_50 = uStack_d8;
            uStack_58 = uStack_e0;
            uStack_60 = uVar43;
            uStack_68 = (ulonglong)plVar3;
            uStack_70 = (ulonglong)plVar23;
            uStack_6 = local_1b8;
            lStack_28 = local_1d0;
            uStack_80 = uVar7;
            uStack_78 = uVar44;
            uStack_40 = uVar34;
            uStack_38 = uVar18;
            lStack_30 = lVar28;
            uStack_18 = uVar27;
            uStack_8 = uVar8;
            uStack_7 = uVar4;
            FUN_ram_00001f40(&local_100);
            uStack_158 = 0x100;
            pplStack_160 = &local_100;
            FUN_ram_0002fb58(&pplStack_160,1);
            uVar9 = 0x1a;
          }
          else {
            uVar27 = (ulonglong)uStack_194;
          }
        }
        else if (plVar25 == (longlong *)0x0) {
          uVar9 = 0;
          uVar27 = 0xbadc0de;
        }
        else {
          if (*(ulonglong *)(lVar38 + 0x710) < 5) {
            uStack_170 = *(ulonglong *)(lVar38 + 0x268) ^ 0x4a1178751b9c3c6;
            uStack_168 = *(ulonglong *)(lVar38 + 0x270) ^ 0x4a0178651b8c3c5;
            uStack_178 = *(ulonglong *)(lVar38 + 0x260) ^ 0x4a2178451bac3c7;
            uStack_180 = *(ulonglong *)(lVar38 + 600) ^ 0xfb5ce87aae443c38;
          }
          else {
            uStack_168 = *(ulonglong *)(lVar38 + 0x270);
            uStack_170 = *(ulonglong *)(lVar38 + 0x268);
            uStack_178 = *(ulonglong *)(lVar38 + 0x260);
            uStack_180 = *(ulonglong *)(lVar38 + 600);
          }
          uStack_148 = *(ulonglong *)(lVar38 + 0x210) ^ 0x4a0178651b8c3c5;
          uStack_150 = *(ulonglong *)(lVar38 + 0x208) ^ 0x4a1178751b9c3c6;
          uStack_158 = *(ulonglong *)(lVar38 + 0x200) ^ 0x4a2178451bac3c7;
          pplStack_160 = (longlong **)(*(ulonglong *)(lVar38 + 0x1f8) ^ 0xfb5ce87aae443c38);
          local_e8 = *(ulonglong *)(lVar38 + 0x1f0) ^ 0x4a0178651b8c3c5;
          local_f0 = (longlong *)(*(ulonglong *)(lVar38 + 0x1e8) ^ 0x4a1178751b9c3c6);
          local_f8 = (longlong *)(*(ulonglong *)(lVar38 + 0x1e0) ^ 0x4a2178451bac3c7);
          local_100 = (longlong *)(*(ulonglong *)(lVar38 + 0x1d8) ^ 0xfb5ce87aae443c38);
          FUN_ram_0000d038(&uStack_190,local_1e8,plVar25,plVar26);
          uVar9 = (ulonglong)uStack_190;
          if (uVar9 == 0x1a) goto LAB_ram_0000f390;
          uVar27 = (ulonglong)uStack_18c;
        }
      }
      else {
        uVar27 = (ulonglong)uStack_184;
      }
    }
  }
  else {
    if (uVar27 == 1) {
      bVar1 = true;
      goto LAB_ram_0000e658;
    }
    uVar27 = 0xbadb100d;
  }
LAB_ram_0000e088:
  uVar41 = (undefined4)uVar27;
LAB_ram_0000e970:
  param_1[1] = uVar41;
  *param_1 = (int)uVar9;
  return;
}

// Function: FUN_ram_0000e500
void FUN_ram_0000e500(void)

{
  undefined1 uVar1;
  bool bVar2;
  bool bVar3;
  longlong lVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  undefined1 uVar8;
  longlong lVar9;
  undefined8 uVar10;
  longlong lVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  undefined8 uVar19;
  ulonglong uVar20;
  undefined8 uVar21;
  undefined8 uVar22;
  undefined8 uVar23;
  undefined8 uVar24;
  longlong lVar25;
  longlong lVar26;
  longlong *plVar27;
  ulonglong uVar28;
  ulonglong unaff_R7;
  ulonglong uVar29;
  undefined4 unaff_R8;
  ulonglong unaff_R9;
  undefined8 uVar30;
  undefined8 local_238;
  ulonglong local_230;
  ulonglong local_228;
  undefined8 local_220;
  ulonglong local_218;
  ulonglong local_210;
  longlong local_208;
  undefined8 *local_200;
  longlong local_1f8;
  undefined4 *local_1f0;
  undefined8 local_1e8;
  longlong local_1e0;
  undefined8 local_1d8;
  longlong local_1d0;
  longlong *local_1c8;
  longlong local_1c0;
  longlong local_1b8;
  undefined8 local_1b0;
  char local_1a8;
  uint local_198;
  uint local_194;
  uint local_190;
  uint local_18c;
  uint local_188;
  uint local_184;
  ulonglong local_180;
  ulonglong local_178;
  ulonglong local_170;
  ulonglong local_168;
  ulonglong *local_160;
  ulonglong local_158;
  ulonglong local_150;
  ulonglong local_148;
  undefined8 local_140;
  undefined8 local_138;
  undefined8 local_130;
  undefined8 local_128;
  longlong local_120;
  undefined8 local_118;
  undefined8 local_110;
  undefined8 local_108;
  ulonglong local_100;
  ulonglong local_f8;
  ulonglong local_f0;
  ulonglong local_e8;
  undefined8 local_e0;
  undefined8 local_d8;
  undefined8 local_d0;
  longlong local_c8;
  ulonglong local_c0;
  ulonglong local_b8;
  ulonglong local_b0;
  ulonglong local_a8;
  ulonglong local_a0;
  ulonglong local_98;
  ulonglong local_90;
  ulonglong local_88;
  undefined8 local_80;
  undefined8 local_78;
  ulonglong local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined8 local_38;
  longlong local_28;
  ulonglong local_20;
  ulonglong local_18;
  ulonglong local_10;
  undefined1 local_7;
  undefined1 local_6;
  undefined1 local_5;
  
  lVar4 = local_1d0;
  lVar9 = FUN_ram_0000e500();
  if (lVar9 != 0) {
    uVar20 = lVar9 << 0x20 | lVar9 - 0x100000000U >> 0x20;
    if (uVar20 < 0x1a) {
      unaff_R8 = *(undefined4 *)(&DAT_ram_00033d50 + uVar20 * 4);
      unaff_R9 = uVar20;
    }
    else {
      unaff_R8 = (undefined4)lVar9;
    }
    goto LAB_ram_0000e970;
  }
  if (unaff_R7 <= local_100) goto LAB_ram_0000e970;
  plVar27 = (longlong *)(local_1e0 + 8);
  uVar8 = FUN_ram_00012848(local_1b0,local_200,plVar27);
  bVar3 = false;
  if (local_1a8 == '\0') {
LAB_ram_0000e658:
    uVar20 = 0xb1ade2;
    unaff_R8 = 0xb1ade2;
    if (0x47 < *(ulonglong *)(local_208 + 0x50)) {
      if (*(ulonglong *)(local_230 + 0x50) < 0x48) goto LAB_ram_0000e970;
      uVar22 = *(undefined8 *)(local_208 + 0x98);
      uVar23 = *(undefined8 *)(local_230 + 0x98);
      local_148 = *(longlong *)(local_1e0 + 0x20);
      local_150 = *(longlong *)(local_1e0 + 0x18);
      local_158 = *(longlong *)(local_1e0 + 0x10);
      local_160 = (ulonglong *)*plVar27;
      FUN_ram_00026e90(&local_100,local_1d0,uVar23,uVar22);
      uVar7 = local_e8;
      uVar6 = local_f0;
      uVar5 = local_f8;
      if ((int)local_100 == 1) {
        uVar20 = local_f8 & 0xffffffff;
        unaff_R9 = local_100 >> 0x20;
      }
      else {
        local_210 = local_1f8 + 0x28;
        uVar20 = local_1f8 + 0x20;
        if (bVar3) {
          local_230 = local_210;
          local_210 = uVar20;
          if (local_1c8 == (longlong *)0x0) {
            local_238 = local_220;
            local_1e8 = local_218;
          }
          else {
            lVar26 = 0x210;
            lVar9 = 0x208;
            lVar11 = 0x200;
            lVar25 = 0x1f8;
            local_1e8 = local_218;
LAB_ram_0000ea48:
            local_170 = *(ulonglong *)(local_1e0 + lVar9) ^ 0x4a1178751b9c3c6;
            local_168 = *(ulonglong *)(local_1e0 + lVar26) ^ 0x4a0178651b8c3c5;
            local_178 = *(ulonglong *)(local_1e0 + lVar11) ^ 0x4a2178451bac3c7;
            local_180 = *(ulonglong *)(local_1e0 + lVar25) ^ 0xfb5ce87aae443c38;
            if (*(ulonglong *)(local_1e0 + 0x710) < 5) {
              local_f0 = *(ulonglong *)(local_1e0 + 0x268) ^ 0x4a1178751b9c3c6;
              local_e8 = *(ulonglong *)(local_1e0 + 0x270) ^ 0x4a0178651b8c3c5;
              local_f8 = *(ulonglong *)(local_1e0 + 0x260) ^ 0x4a2178451bac3c7;
              local_100 = *(ulonglong *)(local_1e0 + 600) ^ 0xfb5ce87aae443c38;
            }
            else {
              local_e8 = *(ulonglong *)(local_1e0 + 0x270);
              local_f0 = *(ulonglong *)(local_1e0 + 0x268);
              local_f8 = *(ulonglong *)(local_1e0 + 0x260);
              local_100 = *(ulonglong *)(local_1e0 + 600);
            }
            FUN_ram_0002a740(&local_160,&local_100,&local_180);
            lVar9 = *local_1c8;
            if ((((*(ulonglong **)(lVar9 + 8) != local_160) ||
                 (*(longlong *)(lVar9 + 0x10) != local_158)) ||
                (*(longlong *)(lVar9 + 0x18) != local_150)) ||
               (bVar2 = false, *(longlong *)(lVar9 + 0x20) != local_148)) {
              bVar2 = true;
            }
            local_238 = local_220;
            if (bVar2) {
              uVar20 = 0xbadc0de8;
              goto LAB_ram_0000e088;
            }
          }
        }
        else {
          local_230 = uVar20;
          if (local_1c8 != (longlong *)0x0) {
            lVar26 = 0x1f0;
            lVar9 = 0x1e8;
            lVar11 = 0x1e0;
            lVar25 = 0x1d8;
            local_1e8 = local_220;
            local_220 = local_218;
            goto LAB_ram_0000ea48;
          }
          local_238 = local_218;
          local_1e8 = local_220;
        }
        local_218 = 0;
        if (local_1c0 != 0) {
          local_218 = (ulonglong)(*(longlong *)(local_1b8 + 8) * local_c8) / 10000;
        }
        if (*(ulonglong *)(local_1e0 + 0x710) < 5) {
          local_170 = *(ulonglong *)(local_1e0 + 0x268) ^ 0x4a1178751b9c3c6;
          local_168 = *(ulonglong *)(local_1e0 + 0x270) ^ 0x4a0178651b8c3c5;
          local_178 = *(ulonglong *)(local_1e0 + 0x260) ^ 0x4a2178451bac3c7;
          local_180 = *(ulonglong *)(local_1e0 + 600) ^ 0xfb5ce87aae443c38;
        }
        else {
          local_168 = *(ulonglong *)(local_1e0 + 0x270);
          local_170 = *(ulonglong *)(local_1e0 + 0x268);
          local_178 = *(ulonglong *)(local_1e0 + 0x260);
          local_180 = *(ulonglong *)(local_1e0 + 600);
        }
        local_148 = *(ulonglong *)(local_1e0 + 0x210) ^ 0x4a0178651b8c3c5;
        local_150 = *(ulonglong *)(local_1e0 + 0x208) ^ 0x4a1178751b9c3c6;
        local_158 = *(ulonglong *)(local_1e0 + 0x200) ^ 0x4a2178451bac3c7;
        local_160 = (ulonglong *)(*(ulonglong *)(local_1e0 + 0x1f8) ^ 0xfb5ce87aae443c38);
        local_e8 = *(ulonglong *)(local_1e0 + 0x1f0) ^ 0x4a0178651b8c3c5;
        local_f0 = *(ulonglong *)(local_1e0 + 0x1e8) ^ 0x4a1178751b9c3c6;
        local_f8 = *(ulonglong *)(local_1e0 + 0x1e0) ^ 0x4a2178451bac3c7;
        local_100 = *(ulonglong *)(local_1e0 + 0x1d8) ^ 0xfb5ce87aae443c38;
        FUN_ram_0000d038(&local_188,local_1e8,local_210,local_1d8);
        unaff_R9 = (ulonglong)local_188;
        if (unaff_R9 == 0x1a) {
          if (local_218 == 0) {
LAB_ram_0000f390:
            local_f8 = local_238;
            local_100 = local_230;
            FUN_ram_0002da18(&local_198,&local_100,8,0);
            unaff_R9 = (ulonglong)local_198;
            if (unaff_R9 == 0x1a) {
              if (!bVar3) {
                local_1d0 = local_c8;
                local_c8 = lVar4;
              }
              local_128 = local_200[3];
              local_130 = local_200[2];
              local_138 = local_200[1];
              local_140 = *local_200;
              local_120 = *plVar27;
              local_118 = *(undefined8 *)(local_1e0 + 0x10);
              local_110 = *(undefined8 *)(local_1e0 + 0x18);
              local_108 = *(undefined8 *)(local_1e0 + 0x20);
              uVar1 = *(undefined1 *)(local_1b8 + 0x18);
              local_1b8._0_1_ = (undefined1)*(undefined8 *)(local_1b8 + 8);
              uVar12 = *(ulonglong *)(local_1e0 + 0x210);
              uVar13 = *(ulonglong *)(local_1e0 + 0x1f0);
              uVar14 = *(ulonglong *)(local_1e0 + 0x208);
              uVar15 = *(ulonglong *)(local_1e0 + 0x1e8);
              uVar16 = *(ulonglong *)(local_1e0 + 0x200);
              uVar17 = *(ulonglong *)(local_1e0 + 0x1e0);
              uVar28 = *(ulonglong *)(local_1e0 + 0x1f8);
              uVar18 = *(ulonglong *)(local_1e0 + 0x1d8);
              uVar29 = *(ulonglong *)(local_1e0 + 0x378);
              uVar20 = *(ulonglong *)(local_1e0 + 800);
              uVar10 = *(undefined8 *)(local_1e0 + 0x298);
              uVar30 = *(undefined8 *)(local_1e0 + 0x2a0);
              uVar19 = *local_200;
              uVar21 = local_200[1];
              uVar24 = local_200[2];
              *(undefined8 *)(local_1e0 + 0x2e0) = local_200[3];
              *(undefined8 *)(local_1e0 + 0x2d8) = uVar24;
              *(undefined8 *)(local_1e0 + 0x2d0) = uVar21;
              *(undefined8 *)(local_1e0 + 0x2c8) = uVar19;
              *(undefined8 *)(local_1e0 + 0x2f0) = uVar30;
              *(undefined8 *)(local_1e0 + 0x2e8) = uVar10;
              *(char *)(local_1e0 + 0x308) = local_1a8;
              *(longlong *)(local_1e0 + 0x300) = local_1d0;
              *(longlong *)(local_1e0 + 0x2f8) = local_c8;
              *(undefined8 *)(local_1e0 + 0x309) = 0;
              *(undefined8 *)(local_1e0 + 0x310) = 0;
              FUN_ram_00002720(local_1e0 + 0x2c8);
              FUN_ram_00031b28(&local_100,&local_140,0x40);
              if (local_1c0 == 0) {
                local_1b8._0_1_ = 0;
              }
              local_20 = local_228 ^ 0x6e9de2b30b19f9ea;
              local_5 = uVar1;
              if (local_1c0 == 0) {
                local_5 = 0;
              }
              uVar20 = uVar20 ^ 0xd3198133b7c1776c;
              local_10 = uVar29 ^ 0x504156a22548f8dd;
              local_c0 = uVar18 ^ 0xfb5ce87aae443c38;
              local_a0 = uVar28 ^ 0xfb5ce87aae443c38;
              local_b8 = uVar17 ^ 0x4a2178451bac3c7;
              local_98 = uVar16 ^ 0x4a2178451bac3c7;
              local_b0 = uVar15 ^ 0x4a1178751b9c3c6;
              local_90 = uVar14 ^ 0x4a1178751b9c3c6;
              local_a8 = uVar13 ^ 0x4a0178651b8c3c5;
              local_88 = uVar12 ^ 0x4a0178651b8c3c5;
              local_48 = local_d0;
              local_50 = local_d8;
              local_58 = local_e0;
              local_60 = uVar7;
              local_68 = uVar6;
              local_70 = uVar5;
              local_6 = (undefined1)local_1b8;
              local_28 = local_1d0;
              local_80 = uVar10;
              local_78 = uVar30;
              local_40 = uVar22;
              local_38 = uVar23;
              local_18 = uVar20;
              local_7 = uVar8;
              FUN_ram_00001f40(&local_100);
              local_158 = 0x100;
              local_160 = &local_100;
              FUN_ram_0002fb58(&local_160,1);
              unaff_R9 = 0x1a;
            }
            else {
              uVar20 = (ulonglong)local_194;
            }
          }
          else if (local_1c8 == (longlong *)0x0) {
            unaff_R9 = 0;
            uVar20 = 0xbadc0de;
          }
          else {
            if (*(ulonglong *)(local_1e0 + 0x710) < 5) {
              local_170 = *(ulonglong *)(local_1e0 + 0x268) ^ 0x4a1178751b9c3c6;
              local_168 = *(ulonglong *)(local_1e0 + 0x270) ^ 0x4a0178651b8c3c5;
              local_178 = *(ulonglong *)(local_1e0 + 0x260) ^ 0x4a2178451bac3c7;
              local_180 = *(ulonglong *)(local_1e0 + 600) ^ 0xfb5ce87aae443c38;
            }
            else {
              local_168 = *(ulonglong *)(local_1e0 + 0x270);
              local_170 = *(ulonglong *)(local_1e0 + 0x268);
              local_178 = *(ulonglong *)(local_1e0 + 0x260);
              local_180 = *(ulonglong *)(local_1e0 + 600);
            }
            local_148 = *(ulonglong *)(local_1e0 + 0x210) ^ 0x4a0178651b8c3c5;
            local_150 = *(ulonglong *)(local_1e0 + 0x208) ^ 0x4a1178751b9c3c6;
            local_158 = *(ulonglong *)(local_1e0 + 0x200) ^ 0x4a2178451bac3c7;
            local_160 = (ulonglong *)(*(ulonglong *)(local_1e0 + 0x1f8) ^ 0xfb5ce87aae443c38);
            local_e8 = *(ulonglong *)(local_1e0 + 0x1f0) ^ 0x4a0178651b8c3c5;
            local_f0 = *(ulonglong *)(local_1e0 + 0x1e8) ^ 0x4a1178751b9c3c6;
            local_f8 = *(ulonglong *)(local_1e0 + 0x1e0) ^ 0x4a2178451bac3c7;
            local_100 = *(ulonglong *)(local_1e0 + 0x1d8) ^ 0xfb5ce87aae443c38;
            FUN_ram_0000d038(&local_190,local_1e8,local_1c8,local_1d8);
            unaff_R9 = (ulonglong)local_190;
            if (unaff_R9 == 0x1a) goto LAB_ram_0000f390;
            uVar20 = (ulonglong)local_18c;
          }
        }
        else {
          uVar20 = (ulonglong)local_184;
        }
      }
    }
  }
  else {
    if (local_1a8 == '\x01') {
      bVar3 = true;
      goto LAB_ram_0000e658;
    }
    uVar20 = 0xbadb100d;
  }
LAB_ram_0000e088:
  unaff_R8 = (undefined4)uVar20;
LAB_ram_0000e970:
  local_1f0[1] = unaff_R8;
  *local_1f0 = (int)unaff_R9;
  return;
}

// Function: FUN_ram_0000f9f8
/* WARNING: Removing unreachable block (ram,0x000103e0) */

void FUN_ram_0000f9f8(undefined4 *param_1,longlong *param_2,undefined8 param_3,undefined8 param_4,
                     longlong param_5)

{
  bool bVar1;
  undefined1 uVar2;
  longlong lVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  undefined8 uVar10;
  undefined1 uVar11;
  longlong lVar12;
  undefined8 uVar13;
  int iVar14;
  longlong *plVar15;
  ulonglong uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  longlong lVar19;
  ulonglong uVar20;
  undefined8 uVar21;
  undefined4 uVar22;
  int iVar23;
  ulonglong uVar24;
  ulonglong uVar25;
  undefined8 *puVar26;
  undefined8 uVar27;
  ulonglong *puVar28;
  ulonglong uVar29;
  longlong lVar30;
  undefined8 uVar31;
  longlong lVar32;
  longlong *plVar33;
  ulonglong uVar34;
  ulonglong uVar35;
  longlong lVar36;
  undefined8 *puVar37;
  ulonglong uVar38;
  ulonglong uVar39;
  longlong *local_218;
  undefined8 local_200;
  longlong *local_1f8;
  longlong *local_1c8;
  undefined8 local_1b0;
  uint uStack_190;
  uint uStack_18c;
  uint uStack_188;
  uint uStack_184;
  ulonglong uStack_180;
  ulonglong uStack_178;
  ulonglong uStack_170;
  ulonglong uStack_168;
  ulonglong *puStack_160;
  ulonglong uStack_158;
  ulonglong uStack_150;
  ulonglong uStack_148;
  undefined8 uStack_140;
  undefined8 uStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 uStack_108;
  ulonglong local_100;
  ulonglong local_f8;
  ulonglong local_f0;
  ulonglong local_e8;
  undefined8 uStack_e0;
  undefined8 uStack_d8;
  undefined8 uStack_d0;
  undefined8 uStack_c8;
  ulonglong uStack_c0;
  ulonglong uStack_b8;
  ulonglong uStack_b0;
  ulonglong uStack_a8;
  ulonglong uStack_a0;
  ulonglong uStack_98;
  ulonglong uStack_90;
  ulonglong uStack_88;
  undefined8 uStack_80;
  undefined8 uStack_78;
  ulonglong uStack_70;
  ulonglong uStack_68;
  ulonglong uStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  ulonglong uStack_20;
  ulonglong uStack_18;
  ulonglong uStack_10;
  undefined1 uStack_8;
  undefined1 uStack_7;
  undefined2 uStack_6;
  
  lVar32 = param_2[9];
  if ((((*(longlong *)(lVar32 + 8) != 0x66d17b1817d5a706) ||
       (*(longlong *)(lVar32 + 0x10) != -0x3f3d02aafb2b25cb)) ||
      (*(longlong *)(lVar32 + 0x18) != -0x5a8aa9de7039db3f)) ||
     (bVar1 = false, *(longlong *)(lVar32 + 0x20) != 0x85fcbbadb)) {
    bVar1 = true;
  }
  uVar35 = 0;
  uVar22 = 0xabad1dea;
  if (bVar1) goto LAB_ram_00010938;
  lVar32 = param_2[6];
  if (((*(longlong *)(lVar32 + 8) != -0x368b38e7e82a58fa) ||
      (*(longlong *)(lVar32 + 0x10) != -0x49a1e296679ca9d8)) ||
     ((*(longlong *)(lVar32 + 0x18) != 0x5c6d4b9ba3b85e8b ||
      (bVar1 = false, *(longlong *)(lVar32 + 0x20) != 0x215b5573)))) {
    bVar1 = true;
  }
  if (bVar1) goto LAB_ram_00010938;
  lVar3 = param_2[1];
  if (((*(longlong *)(lVar3 + 0x28) != -0x16a608d8d48b0286) ||
      (*(longlong *)(lVar3 + 0x30) != 0x7a819dd33c7070c6)) ||
     ((*(longlong *)(lVar3 + 0x38) != 0x6dd2523bce0a93a0 ||
      (bVar1 = false, *(longlong *)(lVar3 + 0x40) != -0x2c4478dc22ab5fac)))) {
    bVar1 = true;
  }
  if (bVar1) goto LAB_ram_00010938;
  if (*(longlong *)(lVar3 + 0x50) != 0x6c0) {
                    /* WARNING: Subroutine does not return */
    FUN_ram_000011b0(&DAT_ram_000337e9,0xe,2);
  }
  if ((lVar3 + 0x58U & 7) != 0) {
                    /* WARNING: Subroutine does not return */
    FUN_ram_000011b0(&DAT_ram_000337e9,0xe,0);
  }
  uVar22 = 0xbadc0de2;
  uVar4 = *(ulonglong *)(lVar3 + 0x288) ^ 0xffffffffffffffa1;
  if (2 < (uVar4 & 0xff)) goto LAB_ram_00010938;
  lVar36 = *param_2;
  puVar37 = (undefined8 *)(lVar36 + 8);
  if ((((*(longlong *)(lVar36 + 8) != 0x77c2575f37eddd1b) ||
       (*(longlong *)(lVar36 + 0x10) != 0x2d6e7a2be59cf048)) ||
      (*(longlong *)(lVar36 + 0x18) != 0x7ea41dd6046c6fc4)) ||
     (bVar1 = false, *(longlong *)(lVar36 + 0x20) != 0x31797eed4f7e7455)) {
    bVar1 = true;
  }
  uVar38 = *(ulonglong *)(lVar3 + 0x2b0);
  uVar24 = *(ulonglong *)(param_5 + -0xff8);
  local_200 = *(undefined8 *)(param_5 + -0x1000);
  if (((bVar1) && (uVar4 = uVar4 & 0xff, uVar4 != 0)) && (uVar4 == 1)) {
    uVar22 = 0x1ced;
    uVar35 = 0;
    goto LAB_ram_00010938;
  }
  plVar15 = param_2 + 3;
  local_1c8 = param_2 + 2;
  uVar16 = *(ulonglong *)(lVar3 + 0x248) ^ 0x4a1178751b9c3c6;
  uVar35 = *(ulonglong *)(lVar3 + 0x250) ^ 0x4a0178651b8c3c5;
  uVar25 = *(ulonglong *)(lVar3 + 0x240) ^ 0x4a2178451bac3c7;
  uVar4 = *(ulonglong *)(lVar3 + 0x238) ^ 0xfb5ce87aae443c38;
  uVar20 = uVar4 << 0x38 | (uVar4 & 0xff00) << 0x28 | (uVar4 & 0xff0000) << 0x18 |
           (uVar4 & 0xff000000) << 8 | uVar4 >> 8 & 0xff000000 | uVar4 >> 0x18 & 0xff0000 |
           uVar4 >> 0x28 & 0xff00 | uVar4 >> 0x38;
  lVar19 = *local_1c8;
  uVar4 = *(ulonglong *)(lVar19 + 8);
  uVar4 = uVar4 << 0x38 | (uVar4 & 0xff00) << 0x28 | (uVar4 & 0xff0000) << 0x18 |
          (uVar4 & 0xff000000) << 8 | uVar4 >> 8 & 0xff000000 | uVar4 >> 0x18 & 0xff0000 |
          uVar4 >> 0x28 & 0xff00 | uVar4 >> 0x38;
  if (uVar20 == uVar4) {
    uVar20 = uVar25 << 0x38 | (uVar25 & 0xff00) << 0x28 | (uVar25 & 0xff0000) << 0x18 |
             (uVar25 & 0xff000000) << 8 | uVar25 >> 8 & 0xff000000 | uVar25 >> 0x18 & 0xff0000 |
             uVar25 >> 0x28 & 0xff00 | uVar25 >> 0x38;
    uVar4 = *(ulonglong *)(lVar19 + 0x10);
    uVar4 = uVar4 << 0x38 | (uVar4 & 0xff00) << 0x28 | (uVar4 & 0xff0000) << 0x18 |
            (uVar4 & 0xff000000) << 8 | uVar4 >> 8 & 0xff000000 | uVar4 >> 0x18 & 0xff0000 |
            uVar4 >> 0x28 & 0xff00 | uVar4 >> 0x38;
    if (uVar20 != uVar4) goto LAB_ram_00010170;
    uVar20 = uVar16 << 0x38 | (uVar16 & 0xff00) << 0x28 | (uVar16 & 0xff0000) << 0x18 |
             (uVar16 & 0xff000000) << 8 | uVar16 >> 8 & 0xff000000 | uVar16 >> 0x18 & 0xff0000 |
             uVar16 >> 0x28 & 0xff00 | uVar16 >> 0x38;
    uVar4 = *(ulonglong *)(lVar19 + 0x18);
    uVar4 = uVar4 << 0x38 | (uVar4 & 0xff00) << 0x28 | (uVar4 & 0xff0000) << 0x18 |
            (uVar4 & 0xff000000) << 8 | uVar4 >> 8 & 0xff000000 | uVar4 >> 0x18 & 0xff0000 |
            uVar4 >> 0x28 & 0xff00 | uVar4 >> 0x38;
    if (uVar20 != uVar4) goto LAB_ram_00010170;
    iVar14 = 0;
    uVar20 = uVar35 << 0x38 | (uVar35 & 0xff00) << 0x28 | (uVar35 & 0xff0000) << 0x18 |
             (uVar35 & 0xff000000) << 8 | uVar35 >> 8 & 0xff000000 | uVar35 >> 0x18 & 0xff0000 |
             uVar35 >> 0x28 & 0xff00 | uVar35 >> 0x38;
    uVar35 = *(ulonglong *)(lVar19 + 0x20);
    uVar4 = uVar35 << 0x38 | (uVar35 & 0xff00) << 0x28 | (uVar35 & 0xff0000) << 0x18 |
            (uVar35 & 0xff000000) << 8 | uVar35 >> 8 & 0xff000000 | uVar35 >> 0x18 & 0xff0000 |
            uVar35 >> 0x28 & 0xff00 | uVar35 >> 0x38;
    if (uVar20 != uVar4) goto LAB_ram_00010170;
  }
  else {
LAB_ram_00010170:
    iVar14 = -1;
    if (uVar4 <= uVar20) {
      iVar14 = 1;
    }
  }
  local_f0 = *(ulonglong *)(lVar3 + 0x228) ^ 0x4a1178751b9c3c6;
  local_e8 = *(ulonglong *)(lVar3 + 0x230) ^ 0x4a0178651b8c3c5;
  local_f8 = *(ulonglong *)(lVar3 + 0x220) ^ 0x4a2178451bac3c7;
  local_100 = *(ulonglong *)(lVar3 + 0x218) ^ 0xfb5ce87aae443c38;
  uVar4 = local_100 << 0x38 | (local_100 & 0xff00) << 0x28 | (local_100 & 0xff0000) << 0x18 |
          (local_100 & 0xff000000) << 8 | local_100 >> 8 & 0xff000000 | local_100 >> 0x18 & 0xff0000
          | local_100 >> 0x28 & 0xff00 | local_100 >> 0x38;
  lVar30 = *plVar15;
  uVar35 = *(ulonglong *)(lVar30 + 8);
  uVar35 = uVar35 << 0x38 | (uVar35 & 0xff00) << 0x28 | (uVar35 & 0xff0000) << 0x18 |
           (uVar35 & 0xff000000) << 8 | uVar35 >> 8 & 0xff000000 | uVar35 >> 0x18 & 0xff0000 |
           uVar35 >> 0x28 & 0xff00 | uVar35 >> 0x38;
  if (uVar4 == uVar35) {
    uVar4 = local_f8 << 0x38 | (local_f8 & 0xff00) << 0x28 | (local_f8 & 0xff0000) << 0x18 |
            (local_f8 & 0xff000000) << 8 | local_f8 >> 8 & 0xff000000 | local_f8 >> 0x18 & 0xff0000
            | local_f8 >> 0x28 & 0xff00 | local_f8 >> 0x38;
    uVar35 = *(ulonglong *)(lVar30 + 0x10);
    uVar35 = uVar35 << 0x38 | (uVar35 & 0xff00) << 0x28 | (uVar35 & 0xff0000) << 0x18 |
             (uVar35 & 0xff000000) << 8 | uVar35 >> 8 & 0xff000000 | uVar35 >> 0x18 & 0xff0000 |
             uVar35 >> 0x28 & 0xff00 | uVar35 >> 0x38;
    if (uVar4 != uVar35) goto LAB_ram_000102d8;
    uVar4 = local_f0 << 0x38 | (local_f0 & 0xff00) << 0x28 | (local_f0 & 0xff0000) << 0x18 |
            (local_f0 & 0xff000000) << 8 | local_f0 >> 8 & 0xff000000 | local_f0 >> 0x18 & 0xff0000
            | local_f0 >> 0x28 & 0xff00 | local_f0 >> 0x38;
    uVar35 = *(ulonglong *)(lVar30 + 0x18);
    uVar35 = uVar35 << 0x38 | (uVar35 & 0xff00) << 0x28 | (uVar35 & 0xff0000) << 0x18 |
             (uVar35 & 0xff000000) << 8 | uVar35 >> 8 & 0xff000000 | uVar35 >> 0x18 & 0xff0000 |
             uVar35 >> 0x28 & 0xff00 | uVar35 >> 0x38;
    if (uVar4 != uVar35) goto LAB_ram_000102d8;
    iVar23 = 0;
    uVar4 = local_e8 << 0x38 | (local_e8 & 0xff00) << 0x28 | (local_e8 & 0xff0000) << 0x18 |
            (local_e8 & 0xff000000) << 8 | local_e8 >> 8 & 0xff000000 | local_e8 >> 0x18 & 0xff0000
            | local_e8 >> 0x28 & 0xff00 | local_e8 >> 0x38;
    uVar35 = *(ulonglong *)(lVar30 + 0x20);
    uVar35 = uVar35 << 0x38 | (uVar35 & 0xff00) << 0x28 | (uVar35 & 0xff0000) << 0x18 |
             (uVar35 & 0xff000000) << 8 | uVar35 >> 8 & 0xff000000 | uVar35 >> 0x18 & 0xff0000 |
             uVar35 >> 0x28 & 0xff00 | uVar35 >> 0x38;
    if (uVar4 != uVar35) goto LAB_ram_000102d8;
  }
  else {
LAB_ram_000102d8:
    iVar23 = -1;
    if (uVar35 <= uVar4) {
      iVar23 = 1;
    }
  }
  if (iVar23 != 0 || iVar14 != 0) {
    uVar22 = 0xbadface3;
    uVar35 = 0;
    goto LAB_ram_00010938;
  }
  uVar4 = *(ulonglong *)(lVar3 + 0x2c0) ^ 0x6e9de2b30b19f1ea;
  uVar16 = *(ulonglong *)(lVar32 + 0x58) - uVar4;
  uVar35 = 0;
  if ((uVar16 <= *(ulonglong *)(lVar32 + 0x58)) &&
     (uVar22 = 0xfaded, (*(ulonglong *)(lVar3 + 0x2b8) ^ 0x6e9de2b30b19f1ea) < uVar16))
  goto LAB_ram_00010938;
  uVar22 = 0xfaded;
  lVar32 = FUN_ram_00010450(&local_100);
  if (lVar32 != 0) {
    uVar4 = lVar32 << 0x20 | lVar32 - 0x100000000U >> 0x20;
    if (uVar4 < 0x1a) {
      uVar22 = *(undefined4 *)(&DAT_ram_00033d50 + uVar4 * 4);
      uVar35 = uVar4;
    }
    else {
      uVar22 = (undefined4)lVar32;
    }
    goto LAB_ram_00010938;
  }
  if (uVar4 <= local_100) goto LAB_ram_00010938;
  puVar26 = (undefined8 *)(lVar3 + 8);
  uVar2 = FUN_ram_00012848(param_4,puVar37,puVar26);
  uVar16 = uVar24 & 0xff;
  uVar4 = uVar35;
  if (uVar16 == 0) {
LAB_ram_000105d0:
    lVar32 = param_2[7];
    if ((((*(longlong *)(lVar32 + 8) != -0x6c5e9a281e0922fa) ||
         (*(longlong *)(lVar32 + 0x10) != -0x53861431b91e3427)) ||
        (*(longlong *)(lVar32 + 0x18) != -0x6ec8a4a0127a4be4)) ||
       (bVar1 = false, *(longlong *)(lVar32 + 0x20) != -0x56ff00817a0a73c6)) {
      bVar1 = true;
    }
    lVar12 = param_2[8];
    if (bVar1) {
      if (((*(longlong *)(lVar32 + 8) != -0x21708a111e0922fa) ||
          (*(longlong *)(lVar32 + 0x10) != -0x2532931b43a2bde8)) ||
         ((*(longlong *)(lVar32 + 0x18) != 0x270db9834dfc1ab6 ||
          (bVar1 = false, *(longlong *)(lVar32 + 0x20) != -0x3745e27d7064202)))) {
        bVar1 = true;
      }
      if (!bVar1) goto LAB_ram_00010720;
    }
    else {
LAB_ram_00010720:
      if (((*(longlong *)(lVar12 + 8) != -0x6c5e9a281e0922fa) ||
          (*(longlong *)(lVar12 + 0x10) != -0x53861431b91e3427)) ||
         ((*(longlong *)(lVar12 + 0x18) != -0x6ec8a4a0127a4be4 ||
          (bVar1 = false, *(longlong *)(lVar12 + 0x20) != -0x56ff00817a0a73c6)))) {
        bVar1 = true;
      }
      if ((!bVar1) ||
         ((((*(longlong *)(lVar12 + 8) == -0x21708a111e0922fa &&
            (*(longlong *)(lVar12 + 0x10) == -0x2532931b43a2bde8)) &&
           (*(longlong *)(lVar12 + 0x18) == 0x270db9834dfc1ab6)) &&
          (*(longlong *)(lVar12 + 0x20) == -0x3745e27d7064202)))) {
        uVar35 = 0;
        uVar22 = 0xb1ade2;
        if ((*(ulonglong *)(lVar19 + 0x50) < 0x48) || (*(ulonglong *)(lVar30 + 0x50) < 0x48))
        goto LAB_ram_00010938;
        uVar27 = *(undefined8 *)(lVar19 + 0x98);
        uVar17 = *(undefined8 *)(lVar30 + 0x98);
        uStack_148 = *(undefined8 *)(lVar3 + 0x20);
        uStack_150 = *(undefined8 *)(lVar3 + 0x18);
        uStack_158 = *(undefined8 *)(lVar3 + 0x10);
        puStack_160 = (ulonglong *)*puVar26;
        FUN_ram_00026e90(&local_100,local_200,uVar17,uVar27);
        uVar25 = local_e8;
        uVar20 = local_f0;
        uVar16 = local_f8;
        if ((int)local_100 == 1) {
          puVar28 = (ulonglong *)(local_f8 & 0xffffffff);
          uVar35 = local_100 >> 0x20;
        }
        else {
          local_218 = param_2 + 5;
          plVar33 = param_2 + 4;
          local_1f8 = plVar15;
          if (uVar4 != 0) {
            plVar33 = local_218;
            local_218 = param_2 + 4;
            local_1f8 = local_1c8;
            local_1c8 = plVar15;
          }
          if (*(ulonglong *)(lVar3 + 0x710) < 5) {
            uStack_170 = *(ulonglong *)(lVar3 + 0x268) ^ 0x4a1178751b9c3c6;
            uStack_168 = *(ulonglong *)(lVar3 + 0x270) ^ 0x4a0178651b8c3c5;
            uStack_178 = *(ulonglong *)(lVar3 + 0x260) ^ 0x4a2178451bac3c7;
            uStack_180 = *(ulonglong *)(lVar3 + 600) ^ 0xfb5ce87aae443c38;
          }
          else {
            uStack_168 = *(ulonglong *)(lVar3 + 0x270);
            uStack_170 = *(ulonglong *)(lVar3 + 0x268);
            uStack_178 = *(ulonglong *)(lVar3 + 0x260);
            uStack_180 = *(ulonglong *)(lVar3 + 600);
          }
          uStack_148 = *(ulonglong *)(lVar3 + 0x210) ^ 0x4a0178651b8c3c5;
          uStack_150 = *(ulonglong *)(lVar3 + 0x208) ^ 0x4a1178751b9c3c6;
          uStack_158 = *(ulonglong *)(lVar3 + 0x200) ^ 0x4a2178451bac3c7;
          puStack_160 = (ulonglong *)(*(ulonglong *)(lVar3 + 0x1f8) ^ 0xfb5ce87aae443c38);
          local_e8 = *(ulonglong *)(lVar3 + 0x1f0) ^ 0x4a0178651b8c3c5;
          local_f0 = *(ulonglong *)(lVar3 + 0x1e8) ^ 0x4a1178751b9c3c6;
          local_f8 = *(ulonglong *)(lVar3 + 0x1e0) ^ 0x4a2178451bac3c7;
          local_100 = *(ulonglong *)(lVar3 + 0x1d8) ^ 0xfb5ce87aae443c38;
          FUN_ram_0000d860(&uStack_188,local_1f8,local_218,param_2 + 1);
          uVar35 = (ulonglong)uStack_188;
          if (uVar35 == 0x1a) {
            FUN_ram_0000cde0(&uStack_190,plVar33,local_1c8,param_2);
            uVar35 = (ulonglong)uStack_190;
            if (uVar35 == 0x1a) {
              local_1b0 = uStack_c8;
              if (uVar4 == 0) {
                local_1b0 = local_200;
              }
              if (uVar4 == 0) {
                local_200 = uStack_c8;
              }
              uStack_128 = *(undefined8 *)(lVar36 + 0x20);
              uStack_130 = *(undefined8 *)(lVar36 + 0x18);
              uStack_138 = *(undefined8 *)(lVar36 + 0x10);
              uStack_140 = *puVar37;
              uStack_120 = *puVar26;
              uStack_118 = *(undefined8 *)(lVar3 + 0x10);
              uStack_110 = *(undefined8 *)(lVar3 + 0x18);
              uStack_108 = *(undefined8 *)(lVar3 + 0x20);
              uVar21 = *puVar37;
              uVar13 = *(undefined8 *)(lVar36 + 0x10);
              uVar18 = *(undefined8 *)(lVar36 + 0x18);
              uVar35 = *(ulonglong *)(lVar3 + 0x1d8);
              uVar4 = *(ulonglong *)(lVar3 + 0x1e0);
              uVar5 = *(ulonglong *)(lVar3 + 0x1e8);
              uVar6 = *(ulonglong *)(lVar3 + 0x1f8);
              uVar7 = *(ulonglong *)(lVar3 + 0x200);
              uVar8 = *(ulonglong *)(lVar3 + 0x208);
              uVar9 = *(ulonglong *)(lVar3 + 0x1f0);
              uVar39 = *(ulonglong *)(lVar3 + 0x210);
              uVar29 = *(ulonglong *)(lVar3 + 0x378);
              uVar34 = *(ulonglong *)(lVar3 + 800);
              uVar10 = *(undefined8 *)(lVar3 + 0x298);
              uVar31 = *(undefined8 *)(lVar3 + 0x2a0);
              *(undefined8 *)(lVar3 + 0x2e0) = *(undefined8 *)(lVar36 + 0x20);
              *(undefined8 *)(lVar3 + 0x2d8) = uVar18;
              *(undefined8 *)(lVar3 + 0x2d0) = uVar13;
              *(undefined8 *)(lVar3 + 0x2c8) = uVar21;
              *(undefined8 *)(lVar3 + 0x2f0) = uVar31;
              *(undefined8 *)(lVar3 + 0x2e8) = uVar10;
              uVar11 = (undefined1)uVar24;
              *(undefined1 *)(lVar3 + 0x308) = uVar11;
              *(undefined8 *)(lVar3 + 0x300) = local_200;
              *(undefined8 *)(lVar3 + 0x2f8) = local_1b0;
              *(undefined8 *)(lVar3 + 0x309) = 0;
              *(undefined8 *)(lVar3 + 0x310) = 0;
              FUN_ram_00002720(lVar3 + 0x2c8);
              puVar28 = &local_100;
              FUN_ram_00031b28(puVar28,&uStack_140,0x40);
              uStack_18 = uVar34 ^ 0xd3198133b7c1776c;
              uStack_10 = uVar29 ^ 0x504156a22548f8dd;
              uStack_88 = uVar39 ^ 0x4a0178651b8c3c5;
              uStack_a8 = uVar9 ^ 0x4a0178651b8c3c5;
              uStack_48 = uStack_d0;
              uStack_50 = uStack_d8;
              uStack_58 = uStack_e0;
              uStack_60 = uVar25;
              uStack_68 = uVar20;
              uStack_70 = uVar16;
              uStack_28 = local_200;
              uStack_30 = local_1b0;
              uStack_90 = uVar8 ^ 0x4a1178751b9c3c6;
              uStack_98 = uVar7 ^ 0x4a2178451bac3c7;
              uStack_a0 = uVar6 ^ 0xfb5ce87aae443c38;
              uStack_b0 = uVar5 ^ 0x4a1178751b9c3c6;
              uStack_b8 = uVar4 ^ 0x4a2178451bac3c7;
              uStack_c0 = uVar35 ^ 0xfb5ce87aae443c38;
              uStack_6 = 0;
              uStack_80 = uVar10;
              uStack_78 = uVar31;
              uStack_40 = uVar27;
              uStack_38 = uVar17;
              uStack_20 = uVar38 ^ 0x6e9de2b30b19f9ea;
              uStack_8 = uVar11;
              uStack_7 = uVar2;
              FUN_ram_00001f40(puVar28);
              uStack_158 = 0x100;
              puStack_160 = puVar28;
              FUN_ram_0002fb58(&puStack_160,1);
              uVar35 = 0x1a;
            }
            else {
              puVar28 = (ulonglong *)(ulonglong)uStack_18c;
            }
          }
          else {
            puVar28 = (ulonglong *)(ulonglong)uStack_184;
          }
        }
        goto LAB_ram_00010880;
      }
    }
    uVar35 = 0;
    uVar22 = 0xbadc0de3;
  }
  else {
    if (uVar16 == 1) {
      uVar4 = 1;
      goto LAB_ram_000105d0;
    }
    puVar28 = (ulonglong *)0xbadb100d;
LAB_ram_00010880:
    uVar22 = SUB84(puVar28,0);
  }
LAB_ram_00010938:
  param_1[1] = uVar22;
  *param_1 = (int)uVar35;
  return;
}

// Function: FUN_ram_00010450
void FUN_ram_00010450(void)

{
  bool bVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  undefined1 uVar5;
  longlong lVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  undefined8 uVar13;
  ulonglong uVar14;
  longlong lVar15;
  undefined8 uVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  undefined4 unaff_R6;
  undefined8 *puVar20;
  undefined8 uVar21;
  ulonglong *puVar22;
  ulonglong uVar23;
  undefined8 uVar24;
  ulonglong uVar25;
  ulonglong unaff_R9;
  ulonglong uVar26;
  longlong local_230;
  longlong local_228;
  longlong local_220;
  ulonglong local_218;
  ulonglong local_210;
  undefined8 local_208;
  undefined8 local_200;
  undefined8 local_1f8;
  undefined8 *local_1e0;
  char local_1d8;
  undefined4 *local_1d0;
  undefined8 local_1c8;
  undefined8 local_1b0;
  longlong local_1a0;
  undefined8 local_198;
  uint local_190;
  uint local_18c;
  uint local_188;
  uint local_184;
  ulonglong local_180;
  ulonglong local_178;
  ulonglong local_170;
  ulonglong local_168;
  ulonglong *local_160;
  ulonglong local_158;
  ulonglong local_150;
  ulonglong local_148;
  undefined8 local_140;
  undefined8 local_138;
  undefined8 local_130;
  undefined8 local_128;
  undefined8 local_120;
  undefined8 local_118;
  undefined8 local_110;
  undefined8 local_108;
  ulonglong local_100;
  ulonglong local_f8;
  ulonglong local_f0;
  ulonglong local_e8;
  undefined8 local_e0;
  undefined8 local_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  ulonglong local_c0;
  ulonglong local_b8;
  ulonglong local_b0;
  ulonglong local_a8;
  ulonglong local_a0;
  ulonglong local_98;
  ulonglong local_90;
  ulonglong local_88;
  undefined8 local_80;
  undefined8 local_78;
  ulonglong local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  ulonglong local_20;
  ulonglong local_18;
  ulonglong local_10;
  undefined1 local_7;
  undefined2 local_6;
  
  lVar6 = FUN_ram_00010450();
  if (lVar6 != 0) {
    uVar14 = lVar6 << 0x20 | lVar6 - 0x100000000U >> 0x20;
    if (uVar14 < 0x1a) {
      unaff_R6 = *(undefined4 *)(&DAT_ram_00033d50 + uVar14 * 4);
      unaff_R9 = uVar14;
    }
    else {
      unaff_R6 = (undefined4)lVar6;
    }
    goto LAB_ram_00010938;
  }
  if (local_218 <= local_100) goto LAB_ram_00010938;
  puVar20 = (undefined8 *)(local_1a0 + 8);
  uVar5 = FUN_ram_00012848(local_198,local_1e0,puVar20);
  uVar14 = unaff_R9;
  if (local_1d8 == '\0') {
LAB_ram_000105d0:
    lVar6 = *(longlong *)(local_230 + 0x38);
    if ((((*(longlong *)(lVar6 + 8) != -0x6c5e9a281e0922fa) ||
         (*(longlong *)(lVar6 + 0x10) != -0x53861431b91e3427)) ||
        (*(longlong *)(lVar6 + 0x18) != -0x6ec8a4a0127a4be4)) ||
       (bVar1 = false, *(longlong *)(lVar6 + 0x20) != -0x56ff00817a0a73c6)) {
      bVar1 = true;
    }
    lVar15 = *(longlong *)(local_230 + 0x40);
    if (bVar1) {
      if (((*(longlong *)(lVar6 + 8) != -0x21708a111e0922fa) ||
          (*(longlong *)(lVar6 + 0x10) != -0x2532931b43a2bde8)) ||
         ((*(longlong *)(lVar6 + 0x18) != 0x270db9834dfc1ab6 ||
          (bVar1 = false, *(longlong *)(lVar6 + 0x20) != -0x3745e27d7064202)))) {
        bVar1 = true;
      }
      if (!bVar1) goto LAB_ram_00010720;
    }
    else {
LAB_ram_00010720:
      if (((*(longlong *)(lVar15 + 8) != -0x6c5e9a281e0922fa) ||
          (*(longlong *)(lVar15 + 0x10) != -0x53861431b91e3427)) ||
         ((*(longlong *)(lVar15 + 0x18) != -0x6ec8a4a0127a4be4 ||
          (bVar1 = false, *(longlong *)(lVar15 + 0x20) != -0x56ff00817a0a73c6)))) {
        bVar1 = true;
      }
      if ((!bVar1) ||
         ((((*(longlong *)(lVar15 + 8) == -0x21708a111e0922fa &&
            (*(longlong *)(lVar15 + 0x10) == -0x2532931b43a2bde8)) &&
           (*(longlong *)(lVar15 + 0x18) == 0x270db9834dfc1ab6)) &&
          (*(longlong *)(lVar15 + 0x20) == -0x3745e27d7064202)))) {
        unaff_R9 = 0;
        unaff_R6 = 0xb1ade2;
        if ((*(ulonglong *)(local_220 + 0x50) < 0x48) || (*(ulonglong *)(local_228 + 0x50) < 0x48))
        goto LAB_ram_00010938;
        uVar21 = *(undefined8 *)(local_220 + 0x98);
        uVar17 = *(undefined8 *)(local_228 + 0x98);
        local_148 = *(undefined8 *)(local_1a0 + 0x20);
        local_150 = *(undefined8 *)(local_1a0 + 0x18);
        local_158 = *(undefined8 *)(local_1a0 + 0x10);
        local_160 = (ulonglong *)*puVar20;
        FUN_ram_00026e90(&local_100,local_200,uVar17,uVar21);
        uVar4 = local_e8;
        uVar3 = local_f0;
        uVar2 = local_f8;
        if ((int)local_100 == 1) {
          puVar22 = (ulonglong *)(local_f8 & 0xffffffff);
          unaff_R9 = local_100 >> 0x20;
        }
        else {
          local_218 = local_230 + 0x28;
          if (uVar14 == 0) {
            local_1f8 = local_1c8;
            local_1c8 = local_208;
            lVar6 = local_230 + 0x20;
          }
          else {
            local_1f8 = local_208;
            lVar6 = local_218;
            local_218 = local_230 + 0x20;
          }
          if (*(ulonglong *)(local_1a0 + 0x710) < 5) {
            local_170 = *(ulonglong *)(local_1a0 + 0x268) ^ 0x4a1178751b9c3c6;
            local_168 = *(ulonglong *)(local_1a0 + 0x270) ^ 0x4a0178651b8c3c5;
            local_178 = *(ulonglong *)(local_1a0 + 0x260) ^ 0x4a2178451bac3c7;
            local_180 = *(ulonglong *)(local_1a0 + 600) ^ 0xfb5ce87aae443c38;
          }
          else {
            local_168 = *(ulonglong *)(local_1a0 + 0x270);
            local_170 = *(ulonglong *)(local_1a0 + 0x268);
            local_178 = *(ulonglong *)(local_1a0 + 0x260);
            local_180 = *(ulonglong *)(local_1a0 + 600);
          }
          local_148 = *(ulonglong *)(local_1a0 + 0x210) ^ 0x4a0178651b8c3c5;
          local_150 = *(ulonglong *)(local_1a0 + 0x208) ^ 0x4a1178751b9c3c6;
          local_158 = *(ulonglong *)(local_1a0 + 0x200) ^ 0x4a2178451bac3c7;
          local_160 = (ulonglong *)(*(ulonglong *)(local_1a0 + 0x1f8) ^ 0xfb5ce87aae443c38);
          local_e8 = *(ulonglong *)(local_1a0 + 0x1f0) ^ 0x4a0178651b8c3c5;
          local_f0 = *(ulonglong *)(local_1a0 + 0x1e8) ^ 0x4a1178751b9c3c6;
          local_f8 = *(ulonglong *)(local_1a0 + 0x1e0) ^ 0x4a2178451bac3c7;
          local_100 = *(ulonglong *)(local_1a0 + 0x1d8) ^ 0xfb5ce87aae443c38;
          FUN_ram_0000d860(&local_188,local_1f8,local_218,local_1b0);
          unaff_R9 = (ulonglong)local_188;
          if (unaff_R9 == 0x1a) {
            FUN_ram_0000cde0(&local_190,lVar6,local_1c8,local_230);
            unaff_R9 = (ulonglong)local_190;
            if (unaff_R9 == 0x1a) {
              local_1b0 = local_c8;
              if (uVar14 == 0) {
                local_1b0 = local_200;
              }
              if (uVar14 == 0) {
                local_200 = local_c8;
              }
              local_128 = local_1e0[3];
              local_130 = local_1e0[2];
              local_138 = local_1e0[1];
              local_140 = *local_1e0;
              local_120 = *puVar20;
              local_118 = *(undefined8 *)(local_1a0 + 0x10);
              local_110 = *(undefined8 *)(local_1a0 + 0x18);
              local_108 = *(undefined8 *)(local_1a0 + 0x20);
              uVar19 = *local_1e0;
              uVar16 = local_1e0[1];
              uVar18 = local_1e0[2];
              uVar14 = *(ulonglong *)(local_1a0 + 0x1d8);
              uVar7 = *(ulonglong *)(local_1a0 + 0x1e0);
              uVar8 = *(ulonglong *)(local_1a0 + 0x1e8);
              uVar9 = *(ulonglong *)(local_1a0 + 0x1f8);
              uVar10 = *(ulonglong *)(local_1a0 + 0x200);
              uVar11 = *(ulonglong *)(local_1a0 + 0x208);
              uVar12 = *(ulonglong *)(local_1a0 + 0x1f0);
              uVar26 = *(ulonglong *)(local_1a0 + 0x210);
              uVar23 = *(ulonglong *)(local_1a0 + 0x378);
              uVar25 = *(ulonglong *)(local_1a0 + 800);
              uVar13 = *(undefined8 *)(local_1a0 + 0x298);
              uVar24 = *(undefined8 *)(local_1a0 + 0x2a0);
              *(undefined8 *)(local_1a0 + 0x2e0) = local_1e0[3];
              *(undefined8 *)(local_1a0 + 0x2d8) = uVar18;
              *(undefined8 *)(local_1a0 + 0x2d0) = uVar16;
              *(undefined8 *)(local_1a0 + 0x2c8) = uVar19;
              *(undefined8 *)(local_1a0 + 0x2f0) = uVar24;
              *(undefined8 *)(local_1a0 + 0x2e8) = uVar13;
              *(char *)(local_1a0 + 0x308) = local_1d8;
              *(undefined8 *)(local_1a0 + 0x300) = local_200;
              *(undefined8 *)(local_1a0 + 0x2f8) = local_1b0;
              *(undefined8 *)(local_1a0 + 0x309) = 0;
              *(undefined8 *)(local_1a0 + 0x310) = 0;
              FUN_ram_00002720(local_1a0 + 0x2c8);
              puVar22 = &local_100;
              FUN_ram_00031b28(puVar22,&local_140,0x40);
              local_18 = uVar25 ^ 0xd3198133b7c1776c;
              local_10 = uVar23 ^ 0x504156a22548f8dd;
              local_88 = uVar26 ^ 0x4a0178651b8c3c5;
              local_a8 = uVar12 ^ 0x4a0178651b8c3c5;
              local_48 = local_d0;
              local_50 = local_d8;
              local_58 = local_e0;
              local_60 = uVar4;
              local_68 = uVar3;
              local_70 = uVar2;
              local_28 = local_200;
              local_30 = local_1b0;
              local_90 = uVar11 ^ 0x4a1178751b9c3c6;
              local_98 = uVar10 ^ 0x4a2178451bac3c7;
              local_a0 = uVar9 ^ 0xfb5ce87aae443c38;
              local_b0 = uVar8 ^ 0x4a1178751b9c3c6;
              local_b8 = uVar7 ^ 0x4a2178451bac3c7;
              local_c0 = uVar14 ^ 0xfb5ce87aae443c38;
              local_6 = 0;
              local_80 = uVar13;
              local_78 = uVar24;
              local_40 = uVar21;
              local_38 = uVar17;
              local_20 = local_210 ^ 0x6e9de2b30b19f9ea;
              local_7 = uVar5;
              FUN_ram_00001f40(puVar22);
              local_158 = 0x100;
              local_160 = puVar22;
              FUN_ram_0002fb58(&local_160,1);
              unaff_R9 = 0x1a;
            }
            else {
              puVar22 = (ulonglong *)(ulonglong)local_18c;
            }
          }
          else {
            puVar22 = (ulonglong *)(ulonglong)local_184;
          }
        }
        goto LAB_ram_00010880;
      }
    }
    unaff_R9 = 0;
    unaff_R6 = 0xbadc0de3;
  }
  else {
    if (local_1d8 == '\x01') {
      uVar14 = 1;
      goto LAB_ram_000105d0;
    }
    puVar22 = (ulonglong *)0xbadb100d;
LAB_ram_00010880:
    unaff_R6 = SUB84(puVar22,0);
  }
LAB_ram_00010938:
  local_1d0[1] = unaff_R6;
  *local_1d0 = (int)unaff_R9;
  return;
}

// Function: FUN_ram_00011490
void FUN_ram_00011490(undefined4 *param_1,longlong param_2,ulonglong param_3,undefined8 *param_4,
                     longlong param_5)

{
  bool bVar1;
  int iVar2;
  byte bVar3;
  undefined4 uVar4;
  int *piVar5;
  char *pcVar6;
  ulonglong *puVar7;
  undefined *puVar8;
  int iVar9;
  longlong *plVar10;
  ulonglong uVar11;
  undefined8 uVar12;
  int iVar13;
  longlong *plVar14;
  longlong lVar15;
  longlong *plVar16;
  int iVar17;
  longlong lVar18;
  byte bVar19;
  ulonglong uVar20;
  byte *pbVar21;
  ulonglong uVar22;
  undefined8 local_90;
  undefined8 local_88;
  ulonglong local_80;
  undefined4 *local_78;
  ulonglong local_70;
  ulonglong local_68;
  ulonglong local_60;
  ulonglong local_58;
  ulonglong local_50;
  ulonglong local_48;
  ulonglong local_40;
  ulonglong local_38;
  ulonglong local_30;
  ulonglong local_28;
  ulonglong local_20;
  ulonglong local_18;
  ulonglong local_10;
  ulonglong local_8;
  
  plVar10 = (longlong *)0xbadc0de1;
  uVar4 = 0;
  local_78 = param_1;
  if ((param_3 < 0xd) || (plVar10 = (longlong *)0xbadc0ded, param_5 != 0x18)) goto LAB_ram_00011c30;
  lVar18 = *(longlong *)(param_2 + 8);
  plVar14 = *(longlong **)(lVar18 + 0x50);
  local_90 = param_3;
  local_88 = param_4;
  local_80 = param_2;
  if (plVar14 < (longlong *)0x6c0) {
    piVar5 = (int *)0x6c0;
    puVar8 = &DAT_ram_00034240;
    plVar16 = plVar14;
    FUN_ram_00031040();
    iVar9 = 0xbadc0de;
    iVar17 = 0;
    if (puVar8 != (undefined *)0x7) goto LAB_ram_00012518;
    if (plVar16 != (longlong *)0x18) {
                    /* WARNING: Subroutine does not return */
      FUN_ram_000011b0(&DAT_ram_000337e9,0xe,2);
    }
    if (((ulonglong)plVar10 & 7) != 0) {
                    /* WARNING: Subroutine does not return */
      FUN_ram_000011b0(&DAT_ram_000337e9,0xe,0);
    }
    lVar18 = *plVar14;
    lVar15 = plVar14[1];
    FUN_ram_00011d88(lVar15 + 0x28,&DAT_ram_00033500,0x20,&local_20);
    iVar2 = (int)local_20;
    local_8 = *(ulonglong *)(lVar15 + 0x270);
    local_10 = *(ulonglong *)(lVar15 + 0x268);
    local_18 = *(ulonglong *)(lVar15 + 0x260);
    local_20 = *(ulonglong *)(lVar15 + 600);
    if (*(ulonglong *)(lVar15 + 0x710) < 5) {
      local_18 = local_18 ^ 0x4a2178451bac3c7;
      local_20 = local_20 ^ 0xfb5ce87aae443c38;
      local_10 = local_10 ^ 0x4a1178751b9c3c6;
      local_8 = local_8 ^ 0x4a0178651b8c3c5;
    }
    uVar22 = local_20 << 0x38 | (local_20 & 0xff00) << 0x28 | (local_20 & 0xff0000) << 0x18 |
             (local_20 & 0xff000000) << 8 | local_20 >> 8 & 0xff000000 | local_20 >> 0x18 & 0xff0000
             | local_20 >> 0x28 & 0xff00 | local_20 >> 0x38;
    uVar11 = *(ulonglong *)(lVar18 + 8);
    uVar11 = uVar11 << 0x38 | (uVar11 & 0xff00) << 0x28 | (uVar11 & 0xff0000) << 0x18 |
             (uVar11 & 0xff000000) << 8 | uVar11 >> 8 & 0xff000000 | uVar11 >> 0x18 & 0xff0000 |
             uVar11 >> 0x28 & 0xff00 | uVar11 >> 0x38;
    if (uVar22 == uVar11) {
      uVar22 = local_18 << 0x38 | (local_18 & 0xff00) << 0x28 | (local_18 & 0xff0000) << 0x18 |
               (local_18 & 0xff000000) << 8 | local_18 >> 8 & 0xff000000 |
               local_18 >> 0x18 & 0xff0000 | local_18 >> 0x28 & 0xff00 | local_18 >> 0x38;
      uVar11 = *(ulonglong *)(lVar18 + 0x10);
      uVar11 = uVar11 << 0x38 | (uVar11 & 0xff00) << 0x28 | (uVar11 & 0xff0000) << 0x18 |
               (uVar11 & 0xff000000) << 8 | uVar11 >> 8 & 0xff000000 | uVar11 >> 0x18 & 0xff0000 |
               uVar11 >> 0x28 & 0xff00 | uVar11 >> 0x38;
      if (uVar22 != uVar11) goto LAB_ram_00011f78;
      uVar22 = local_10 << 0x38 | (local_10 & 0xff00) << 0x28 | (local_10 & 0xff0000) << 0x18 |
               (local_10 & 0xff000000) << 8 | local_10 >> 8 & 0xff000000 |
               local_10 >> 0x18 & 0xff0000 | local_10 >> 0x28 & 0xff00 | local_10 >> 0x38;
      uVar11 = *(ulonglong *)(lVar18 + 0x18);
      uVar11 = uVar11 << 0x38 | (uVar11 & 0xff00) << 0x28 | (uVar11 & 0xff0000) << 0x18 |
               (uVar11 & 0xff000000) << 8 | uVar11 >> 8 & 0xff000000 | uVar11 >> 0x18 & 0xff0000 |
               uVar11 >> 0x28 & 0xff00 | uVar11 >> 0x38;
      if (uVar22 != uVar11) goto LAB_ram_00011f78;
      iVar13 = 0;
      uVar22 = local_8 << 0x38 | (local_8 & 0xff00) << 0x28 | (local_8 & 0xff0000) << 0x18 |
               (local_8 & 0xff000000) << 8 | local_8 >> 8 & 0xff000000 | local_8 >> 0x18 & 0xff0000
               | local_8 >> 0x28 & 0xff00 | local_8 >> 0x38;
      uVar11 = *(ulonglong *)(lVar18 + 0x20);
      uVar11 = uVar11 << 0x38 | (uVar11 & 0xff00) << 0x28 | (uVar11 & 0xff0000) << 0x18 |
               (uVar11 & 0xff000000) << 8 | uVar11 >> 8 & 0xff000000 | uVar11 >> 0x18 & 0xff0000 |
               uVar11 >> 0x28 & 0xff00 | uVar11 >> 0x38;
      if (uVar22 != uVar11) goto LAB_ram_00011f78;
    }
    else {
LAB_ram_00011f78:
      iVar13 = -1;
      if (uVar11 <= uVar22) {
        iVar13 = 1;
      }
    }
    iVar9 = -0x5452e216;
    iVar17 = 0;
    if ((*(char *)(lVar18 + 1) != '\0') && (iVar13 == 0 && iVar2 == 0)) {
      local_68 = *(ulonglong *)(lVar15 + 0x250) ^ 0x4a0178651b8c3c5;
      local_70 = *(ulonglong *)(lVar15 + 0x248) ^ 0x4a1178751b9c3c6;
      local_78 = (undefined4 *)(*(ulonglong *)(lVar15 + 0x240) ^ 0x4a2178451bac3c7);
      local_80 = *(ulonglong *)(lVar15 + 0x238) ^ 0xfb5ce87aae443c38;
      local_48 = *(ulonglong *)(lVar15 + 0x230) ^ 0x4a0178651b8c3c5;
      local_50 = *(ulonglong *)(lVar15 + 0x228) ^ 0x4a1178751b9c3c6;
      local_58 = *(ulonglong *)(lVar15 + 0x220) ^ 0x4a2178451bac3c7;
      local_60 = *(ulonglong *)(lVar15 + 0x218) ^ 0xfb5ce87aae443c38;
      lVar18 = plVar14[2];
      if ((local_80 != *(ulonglong *)(lVar18 + 8)) ||
         (((local_78 != (undefined4 *)*(ulonglong *)(lVar18 + 0x10) ||
           (local_70 != *(ulonglong *)(lVar18 + 0x18))) ||
          (bVar1 = false, local_68 != *(ulonglong *)(lVar18 + 0x20))))) {
        bVar1 = true;
      }
      iVar17 = 0;
      iVar9 = -0x4520531d;
      if (!bVar1) {
        lVar18 = plVar14[4];
        if (((local_60 != *(ulonglong *)(lVar18 + 8)) || (local_58 != *(ulonglong *)(lVar18 + 0x10))
            ) || ((local_50 != *(ulonglong *)(lVar18 + 0x18) ||
                  (bVar1 = false, local_48 != *(ulonglong *)(lVar18 + 0x20))))) {
          bVar1 = true;
        }
        if (!bVar1) {
          local_28 = *(ulonglong *)(lVar15 + 0x210) ^ 0x4a0178651b8c3c5;
          local_30 = *(ulonglong *)(lVar15 + 0x208) ^ 0x4a1178751b9c3c6;
          local_38 = *(ulonglong *)(lVar15 + 0x200) ^ 0x4a2178451bac3c7;
          local_40 = *(ulonglong *)(lVar15 + 0x1f8) ^ 0xfb5ce87aae443c38;
          local_8 = *(ulonglong *)(lVar15 + 0x1f0) ^ 0x4a0178651b8c3c5;
          local_10 = *(ulonglong *)(lVar15 + 0x1e8) ^ 0x4a1178751b9c3c6;
          local_18 = *(ulonglong *)(lVar15 + 0x1e0) ^ 0x4a2178451bac3c7;
          local_20 = *(ulonglong *)(lVar15 + 0x1d8) ^ 0xfb5ce87aae443c38;
          if (*plVar10 != 0) {
            FUN_ram_0000d038(&local_88,plVar14 + 2,plVar14 + 3,plVar14 + 1);
            if ((int)local_88 != 0x1a) {
              iVar9 = local_88._4_4_;
              iVar17 = (int)local_88;
              goto LAB_ram_00012518;
            }
          }
          iVar9 = (int)(plVar14 + 1);
          if (plVar10[1] != 0) {
            FUN_ram_0000d038(&local_90,plVar14 + 4,plVar14 + 5);
            if ((int)local_90 != 0x1a) {
              iVar9 = local_90._4_4_;
              iVar17 = (int)local_90;
              goto LAB_ram_00012518;
            }
          }
          *(ulonglong *)(lVar15 + 0x2a8) = plVar10[2] ^ 0x6e9de2b30b19f9ea;
          iVar17 = 0x1a;
        }
      }
    }
LAB_ram_00012518:
    piVar5[1] = iVar9;
    *piVar5 = iVar17;
    return;
  }
  if ((lVar18 + 0x58U & 7) != 0) goto LAB_ram_00011ca0;
  local_38 = *(ulonglong *)(lVar18 + 0x3c0) ^ 0x9da8833b65a9ba;
  local_30 = *(ulonglong *)(lVar18 + 0x3c8) ^ 0x9ca8823b66a9b9;
  local_28 = *(ulonglong *)(lVar18 + 0x3d0) ^ 0x93a88d3b67a9b8;
  local_60 = *(ulonglong *)(lVar18 + 0x398) ^ 0x9aa8843b60a9bf;
  local_58 = *(ulonglong *)(lVar18 + 0x3a0) ^ 0x99a8873b61a9be;
  local_50 = *(ulonglong *)(lVar18 + 0x3a8) ^ 0x98a8863b62a9bd;
  local_48 = *(ulonglong *)(lVar18 + 0x3b0) ^ 0x9fa8813b63a9bc;
  local_40 = *(ulonglong *)(lVar18 + 0x3b8) ^ 0x9ea8803b64a9bb;
  local_20 = *(ulonglong *)(lVar18 + 0x3d8) ^ 0x92a88c3b68a9b7;
  local_68 = *(ulonglong *)(lVar18 + 0x390) ^ 0xff64577ac49fae40;
  bVar3 = 1;
  bVar19 = 1;
  if (*(ulonglong *)(lVar18 + 0x390) == 0xff64577ac49fae40) {
    bVar19 = 0;
    if (6 < *(ulonglong *)(lVar18 + 0x710)) goto LAB_ram_000116f8;
LAB_ram_00011738:
    bVar3 = 0;
    lVar18 = *(longlong *)(param_2 + 0x60);
    if (*(longlong *)(lVar18 + 0x28) != 0) goto LAB_ram_00011790;
LAB_ram_00011760:
    if (((*(longlong *)(lVar18 + 0x30) != 0) || (*(longlong *)(lVar18 + 0x38) != 0)) ||
       (*(longlong *)(lVar18 + 0x40) != 0)) goto LAB_ram_00011790;
LAB_ram_00011b48:
    local_8 = 0;
    local_10 = 0;
    local_18 = 1;
    uVar22 = (ulonglong)param_4 & 7;
joined_r0x00011c98:
    if (uVar22 != 0) {
LAB_ram_00011ca0:
                    /* WARNING: Subroutine does not return */
      FUN_ram_000011b0(&DAT_ram_000337df,10,0);
    }
    FUN_ram_0000f9f8(&local_70,local_80,local_90,*local_88);
    plVar10 = (longlong *)(local_70 >> 0x20);
    uVar4 = (undefined4)local_70;
  }
  else {
    if (*(ulonglong *)(lVar18 + 0x710) < 7) goto LAB_ram_00011738;
LAB_ram_000116f8:
    lVar18 = *(longlong *)(param_2 + 0x60);
    if (*(longlong *)(lVar18 + 0x28) == 0) goto LAB_ram_00011760;
LAB_ram_00011790:
    if ((((*(longlong *)(lVar18 + 0x28) != 0x164b2e264bbb7a0a) ||
         (*(longlong *)(lVar18 + 0x30) != -0x365741ee0508e6a4)) ||
        (*(longlong *)(lVar18 + 0x38) != 0x7f02a960c3f7095f)) ||
       (bVar1 = false, *(longlong *)(lVar18 + 0x40) != -0x5c64a878aed5b343)) {
      bVar1 = true;
    }
    if ((!bVar1) && (0x57 < *(ulonglong *)(lVar18 + 0x50))) {
      pcVar6 = (char *)(lVar18 + 0x58);
      if (((ulonglong)pcVar6 & 7) != 0) goto LAB_ram_00011ca0;
      if (*pcVar6 == '\x02') {
        lVar15 = *(longlong *)(param_2 + 0x48);
        if (((*(longlong *)(lVar15 + 8) != 0x66d17b1817d5a706) ||
            (*(longlong *)(lVar15 + 0x10) != -0x3f3d02aafb2b25cb)) ||
           ((*(longlong *)(lVar15 + 0x18) != -0x5a8aa9de7039db3f ||
            (bVar1 = false, *(longlong *)(lVar15 + 0x20) != 0x85fcbbadb)))) {
          bVar1 = true;
        }
        if (!bVar1) {
          if ((ulonglong)*(ushort *)(lVar15 + 0x58) != 0) {
            uVar22 = 0;
            do {
              uVar11 = (ulonglong)*(ushort *)(lVar15 + 0x5a + uVar22 * 2);
              uVar20 = (ulonglong)*(ushort *)(lVar15 + 0x58 + uVar11);
              if (uVar20 != 0) {
                pbVar21 = (byte *)(lVar15 + 0x5a + uVar11);
                uVar11 = 0;
                do {
                  if ((*pbVar21 & 1) != 0) {
                    if ((((*(longlong *)(pbVar21 + 1) != *(longlong *)(lVar18 + 100)) ||
                         (*(longlong *)(pbVar21 + 9) != *(longlong *)(lVar18 + 0x6c))) ||
                        (*(longlong *)(pbVar21 + 0x11) != *(longlong *)(lVar18 + 0x74))) ||
                       (bVar1 = false, *(longlong *)(pbVar21 + 0x19) != *(longlong *)(lVar18 + 0x7c)
                       )) {
                      bVar1 = true;
                    }
                    if (!bVar1) {
                      if ((bool)(bVar3 & bVar19)) {
                        puVar7 = &local_68;
                        uVar12 = 5;
                      }
                      else {
                        puVar7 = (ulonglong *)0x8;
                        uVar12 = 0;
                      }
                      FUN_ram_0002d050(&local_18,pcVar6,puVar7,uVar12);
                      uVar22 = (ulonglong)local_88 & 7;
                      goto joined_r0x00011c98;
                    }
                  }
                  pbVar21 = pbVar21 + 0x21;
                  uVar11 = uVar11 + 1;
                } while (uVar11 < uVar20);
              }
              uVar22 = uVar22 + 1;
            } while (uVar22 < *(ushort *)(lVar15 + 0x58));
          }
          goto LAB_ram_00011b48;
        }
      }
    }
    plVar10 = (longlong *)0xabad1dea;
    uVar4 = 0;
  }
LAB_ram_00011c30:
  local_78[1] = (int)plVar10;
  *local_78 = uVar4;
  return;
}

// Function: FUN_ram_00011cf0
void FUN_ram_00011cf0(int *param_1,longlong *param_2,longlong param_3,longlong *param_4,
                     longlong param_5)

{
  bool bVar1;
  ulonglong uVar2;
  int iVar3;
  int iVar4;
  ulonglong uVar5;
  int iVar6;
  int iVar7;
  longlong lVar8;
  longlong lVar9;
  int iStack_90;
  int iStack_8c;
  int iStack_88;
  int iStack_84;
  ulonglong uStack_80;
  ulonglong uStack_78;
  ulonglong uStack_70;
  ulonglong uStack_68;
  ulonglong uStack_60;
  ulonglong uStack_58;
  ulonglong uStack_50;
  ulonglong uStack_48;
  ulonglong uStack_40;
  ulonglong uStack_38;
  ulonglong uStack_30;
  ulonglong uStack_28;
  ulonglong uStack_20;
  ulonglong uStack_18;
  ulonglong uStack_10;
  ulonglong uStack_8;
  
  iVar3 = 0xbadc0de;
  iVar7 = 0;
  if (param_3 != 7) goto LAB_ram_00012518;
  if (param_5 != 0x18) {
                    /* WARNING: Subroutine does not return */
    FUN_ram_000011b0(&DAT_ram_000337e9,0xe,2);
  }
  if (((ulonglong)param_4 & 7) != 0) {
                    /* WARNING: Subroutine does not return */
    FUN_ram_000011b0(&DAT_ram_000337e9,0xe,0);
  }
  lVar8 = *param_2;
  lVar9 = param_2[1];
  FUN_ram_00011d88(lVar9 + 0x28,&DAT_ram_00033500,0x20,&uStack_20);
  iVar4 = (int)uStack_20;
  uStack_8 = *(ulonglong *)(lVar9 + 0x270);
  uStack_10 = *(ulonglong *)(lVar9 + 0x268);
  uStack_18 = *(ulonglong *)(lVar9 + 0x260);
  uStack_20 = *(ulonglong *)(lVar9 + 600);
  if (*(ulonglong *)(lVar9 + 0x710) < 5) {
    uStack_18 = uStack_18 ^ 0x4a2178451bac3c7;
    uStack_20 = uStack_20 ^ 0xfb5ce87aae443c38;
    uStack_10 = uStack_10 ^ 0x4a1178751b9c3c6;
    uStack_8 = uStack_8 ^ 0x4a0178651b8c3c5;
  }
  uVar2 = uStack_20 << 0x38 | (uStack_20 & 0xff00) << 0x28 | (uStack_20 & 0xff0000) << 0x18 |
          (uStack_20 & 0xff000000) << 8 | uStack_20 >> 8 & 0xff000000 | uStack_20 >> 0x18 & 0xff0000
          | uStack_20 >> 0x28 & 0xff00 | uStack_20 >> 0x38;
  uVar5 = *(ulonglong *)(lVar8 + 8);
  uVar5 = uVar5 << 0x38 | (uVar5 & 0xff00) << 0x28 | (uVar5 & 0xff0000) << 0x18 |
          (uVar5 & 0xff000000) << 8 | uVar5 >> 8 & 0xff000000 | uVar5 >> 0x18 & 0xff0000 |
          uVar5 >> 0x28 & 0xff00 | uVar5 >> 0x38;
  if (uVar2 == uVar5) {
    uVar2 = uStack_18 << 0x38 | (uStack_18 & 0xff00) << 0x28 | (uStack_18 & 0xff0000) << 0x18 |
            (uStack_18 & 0xff000000) << 8 | uStack_18 >> 8 & 0xff000000 |
            uStack_18 >> 0x18 & 0xff0000 | uStack_18 >> 0x28 & 0xff00 | uStack_18 >> 0x38;
    uVar5 = *(ulonglong *)(lVar8 + 0x10);
    uVar5 = uVar5 << 0x38 | (uVar5 & 0xff00) << 0x28 | (uVar5 & 0xff0000) << 0x18 |
            (uVar5 & 0xff000000) << 8 | uVar5 >> 8 & 0xff000000 | uVar5 >> 0x18 & 0xff0000 |
            uVar5 >> 0x28 & 0xff00 | uVar5 >> 0x38;
    if (uVar2 != uVar5) goto LAB_ram_00011f78;
    uVar2 = uStack_10 << 0x38 | (uStack_10 & 0xff00) << 0x28 | (uStack_10 & 0xff0000) << 0x18 |
            (uStack_10 & 0xff000000) << 8 | uStack_10 >> 8 & 0xff000000 |
            uStack_10 >> 0x18 & 0xff0000 | uStack_10 >> 0x28 & 0xff00 | uStack_10 >> 0x38;
    uVar5 = *(ulonglong *)(lVar8 + 0x18);
    uVar5 = uVar5 << 0x38 | (uVar5 & 0xff00) << 0x28 | (uVar5 & 0xff0000) << 0x18 |
            (uVar5 & 0xff000000) << 8 | uVar5 >> 8 & 0xff000000 | uVar5 >> 0x18 & 0xff0000 |
            uVar5 >> 0x28 & 0xff00 | uVar5 >> 0x38;
    if (uVar2 != uVar5) goto LAB_ram_00011f78;
    iVar6 = 0;
    uVar2 = uStack_8 << 0x38 | (uStack_8 & 0xff00) << 0x28 | (uStack_8 & 0xff0000) << 0x18 |
            (uStack_8 & 0xff000000) << 8 | uStack_8 >> 8 & 0xff000000 | uStack_8 >> 0x18 & 0xff0000
            | uStack_8 >> 0x28 & 0xff00 | uStack_8 >> 0x38;
    uVar5 = *(ulonglong *)(lVar8 + 0x20);
    uVar5 = uVar5 << 0x38 | (uVar5 & 0xff00) << 0x28 | (uVar5 & 0xff0000) << 0x18 |
            (uVar5 & 0xff000000) << 8 | uVar5 >> 8 & 0xff000000 | uVar5 >> 0x18 & 0xff0000 |
            uVar5 >> 0x28 & 0xff00 | uVar5 >> 0x38;
    if (uVar2 != uVar5) goto LAB_ram_00011f78;
  }
  else {
LAB_ram_00011f78:
    iVar6 = -1;
    if (uVar5 <= uVar2) {
      iVar6 = 1;
    }
  }
  iVar3 = -0x5452e216;
  iVar7 = 0;
  if ((*(char *)(lVar8 + 1) != '\0') && (iVar6 == 0 && iVar4 == 0)) {
    uStack_68 = *(ulonglong *)(lVar9 + 0x250) ^ 0x4a0178651b8c3c5;
    uStack_70 = *(ulonglong *)(lVar9 + 0x248) ^ 0x4a1178751b9c3c6;
    uStack_78 = *(ulonglong *)(lVar9 + 0x240) ^ 0x4a2178451bac3c7;
    uStack_80 = *(ulonglong *)(lVar9 + 0x238) ^ 0xfb5ce87aae443c38;
    uStack_48 = *(ulonglong *)(lVar9 + 0x230) ^ 0x4a0178651b8c3c5;
    uStack_50 = *(ulonglong *)(lVar9 + 0x228) ^ 0x4a1178751b9c3c6;
    uStack_58 = *(ulonglong *)(lVar9 + 0x220) ^ 0x4a2178451bac3c7;
    uStack_60 = *(ulonglong *)(lVar9 + 0x218) ^ 0xfb5ce87aae443c38;
    lVar8 = param_2[2];
    if ((uStack_80 != *(ulonglong *)(lVar8 + 8)) ||
       (((uStack_78 != *(ulonglong *)(lVar8 + 0x10) || (uStack_70 != *(ulonglong *)(lVar8 + 0x18)))
        || (bVar1 = false, uStack_68 != *(ulonglong *)(lVar8 + 0x20))))) {
      bVar1 = true;
    }
    iVar7 = 0;
    iVar3 = -0x4520531d;
    if (!bVar1) {
      lVar8 = param_2[4];
      if (((uStack_60 != *(ulonglong *)(lVar8 + 8)) || (uStack_58 != *(ulonglong *)(lVar8 + 0x10)))
         || ((uStack_50 != *(ulonglong *)(lVar8 + 0x18) ||
             (bVar1 = false, uStack_48 != *(ulonglong *)(lVar8 + 0x20))))) {
        bVar1 = true;
      }
      if (!bVar1) {
        uStack_28 = *(ulonglong *)(lVar9 + 0x210) ^ 0x4a0178651b8c3c5;
        uStack_30 = *(ulonglong *)(lVar9 + 0x208) ^ 0x4a1178751b9c3c6;
        uStack_38 = *(ulonglong *)(lVar9 + 0x200) ^ 0x4a2178451bac3c7;
        uStack_40 = *(ulonglong *)(lVar9 + 0x1f8) ^ 0xfb5ce87aae443c38;
        uStack_8 = *(ulonglong *)(lVar9 + 0x1f0) ^ 0x4a0178651b8c3c5;
        uStack_10 = *(ulonglong *)(lVar9 + 0x1e8) ^ 0x4a1178751b9c3c6;
        uStack_18 = *(ulonglong *)(lVar9 + 0x1e0) ^ 0x4a2178451bac3c7;
        uStack_20 = *(ulonglong *)(lVar9 + 0x1d8) ^ 0xfb5ce87aae443c38;
        if (((*param_4 == 0) ||
            (FUN_ram_0000d038(&iStack_88,param_2 + 2,param_2 + 3,param_2 + 1), iVar3 = iStack_84,
            iVar7 = iStack_88, iStack_88 == 0x1a)) &&
           ((iVar4 = (int)(param_2 + 1), param_4[1] == 0 ||
            (FUN_ram_0000d038(&iStack_90,param_2 + 4,param_2 + 5), iVar3 = iStack_8c,
            iVar7 = iStack_90, iStack_90 == 0x1a)))) {
          *(ulonglong *)(lVar9 + 0x2a8) = param_4[2] ^ 0x6e9de2b30b19f9ea;
          iVar3 = iVar4;
          iVar7 = 0x1a;
        }
      }
    }
  }
LAB_ram_00012518:
  param_1[1] = iVar3;
  *param_1 = iVar7;
  return;
}

// Function: FUN_ram_00011d88
void FUN_ram_00011d88(void)

{
  bool bVar1;
  ulonglong uVar2;
  longlong lVar3;
  int iVar4;
  int iVar5;
  ulonglong uVar6;
  int iVar7;
  int iVar8;
  longlong unaff_R6;
  longlong unaff_R7;
  longlong *unaff_R8;
  int *unaff_R9;
  longlong local_a0;
  int local_90;
  int local_8c;
  int local_88;
  int local_84;
  ulonglong local_80;
  ulonglong local_78;
  ulonglong local_70;
  ulonglong local_68;
  ulonglong local_60;
  ulonglong local_58;
  ulonglong local_50;
  ulonglong local_48;
  ulonglong local_40;
  ulonglong local_38;
  ulonglong local_30;
  ulonglong local_28;
  ulonglong local_20;
  ulonglong local_18;
  ulonglong local_10;
  ulonglong local_8;
  
  FUN_ram_00011d88();
  iVar5 = (int)local_20;
  local_8 = *(ulonglong *)(unaff_R7 + 0x270);
  local_10 = *(ulonglong *)(unaff_R7 + 0x268);
  local_18 = *(ulonglong *)(unaff_R7 + 0x260);
  local_20 = *(ulonglong *)(unaff_R7 + 600);
  if (*(ulonglong *)(unaff_R7 + 0x710) < 5) {
    local_18 = local_18 ^ 0x4a2178451bac3c7;
    local_20 = local_20 ^ 0xfb5ce87aae443c38;
    local_10 = local_10 ^ 0x4a1178751b9c3c6;
    local_8 = local_8 ^ 0x4a0178651b8c3c5;
  }
  uVar2 = local_20 << 0x38 | (local_20 & 0xff00) << 0x28 | (local_20 & 0xff0000) << 0x18 |
          (local_20 & 0xff000000) << 8 | local_20 >> 8 & 0xff000000 | local_20 >> 0x18 & 0xff0000 |
          local_20 >> 0x28 & 0xff00 | local_20 >> 0x38;
  uVar6 = *(ulonglong *)(unaff_R6 + 8);
  uVar6 = uVar6 << 0x38 | (uVar6 & 0xff00) << 0x28 | (uVar6 & 0xff0000) << 0x18 |
          (uVar6 & 0xff000000) << 8 | uVar6 >> 8 & 0xff000000 | uVar6 >> 0x18 & 0xff0000 |
          uVar6 >> 0x28 & 0xff00 | uVar6 >> 0x38;
  if (uVar2 == uVar6) {
    uVar2 = local_18 << 0x38 | (local_18 & 0xff00) << 0x28 | (local_18 & 0xff0000) << 0x18 |
            (local_18 & 0xff000000) << 8 | local_18 >> 8 & 0xff000000 | local_18 >> 0x18 & 0xff0000
            | local_18 >> 0x28 & 0xff00 | local_18 >> 0x38;
    uVar6 = *(ulonglong *)(unaff_R6 + 0x10);
    uVar6 = uVar6 << 0x38 | (uVar6 & 0xff00) << 0x28 | (uVar6 & 0xff0000) << 0x18 |
            (uVar6 & 0xff000000) << 8 | uVar6 >> 8 & 0xff000000 | uVar6 >> 0x18 & 0xff0000 |
            uVar6 >> 0x28 & 0xff00 | uVar6 >> 0x38;
    if (uVar2 == uVar6) {
      uVar2 = local_10 << 0x38 | (local_10 & 0xff00) << 0x28 | (local_10 & 0xff0000) << 0x18 |
              (local_10 & 0xff000000) << 8 | local_10 >> 8 & 0xff000000 |
              local_10 >> 0x18 & 0xff0000 | local_10 >> 0x28 & 0xff00 | local_10 >> 0x38;
      uVar6 = *(ulonglong *)(unaff_R6 + 0x18);
      uVar6 = uVar6 << 0x38 | (uVar6 & 0xff00) << 0x28 | (uVar6 & 0xff0000) << 0x18 |
              (uVar6 & 0xff000000) << 8 | uVar6 >> 8 & 0xff000000 | uVar6 >> 0x18 & 0xff0000 |
              uVar6 >> 0x28 & 0xff00 | uVar6 >> 0x38;
      if (uVar2 == uVar6) {
        iVar7 = 0;
        uVar2 = local_8 << 0x38 | (local_8 & 0xff00) << 0x28 | (local_8 & 0xff0000) << 0x18 |
                (local_8 & 0xff000000) << 8 | local_8 >> 8 & 0xff000000 | local_8 >> 0x18 & 0xff0000
                | local_8 >> 0x28 & 0xff00 | local_8 >> 0x38;
        uVar6 = *(ulonglong *)(unaff_R6 + 0x20);
        uVar6 = uVar6 << 0x38 | (uVar6 & 0xff00) << 0x28 | (uVar6 & 0xff0000) << 0x18 |
                (uVar6 & 0xff000000) << 8 | uVar6 >> 8 & 0xff000000 | uVar6 >> 0x18 & 0xff0000 |
                uVar6 >> 0x28 & 0xff00 | uVar6 >> 0x38;
        if (uVar2 == uVar6) goto LAB_ram_00011f90;
      }
    }
  }
  iVar7 = -1;
  if (uVar6 <= uVar2) {
    iVar7 = 1;
  }
LAB_ram_00011f90:
  iVar4 = -0x5452e216;
  iVar8 = 0;
  if ((*(char *)(unaff_R6 + 1) != '\0') && (iVar7 == 0 && iVar5 == 0)) {
    local_68 = *(ulonglong *)(unaff_R7 + 0x250) ^ 0x4a0178651b8c3c5;
    local_70 = *(ulonglong *)(unaff_R7 + 0x248) ^ 0x4a1178751b9c3c6;
    local_78 = *(ulonglong *)(unaff_R7 + 0x240) ^ 0x4a2178451bac3c7;
    local_80 = *(ulonglong *)(unaff_R7 + 0x238) ^ 0xfb5ce87aae443c38;
    local_48 = *(ulonglong *)(unaff_R7 + 0x230) ^ 0x4a0178651b8c3c5;
    local_50 = *(ulonglong *)(unaff_R7 + 0x228) ^ 0x4a1178751b9c3c6;
    local_58 = *(ulonglong *)(unaff_R7 + 0x220) ^ 0x4a2178451bac3c7;
    local_60 = *(ulonglong *)(unaff_R7 + 0x218) ^ 0xfb5ce87aae443c38;
    lVar3 = *(longlong *)(local_a0 + 0x10);
    if ((local_80 != *(ulonglong *)(lVar3 + 8)) ||
       (((local_78 != *(ulonglong *)(lVar3 + 0x10) || (local_70 != *(ulonglong *)(lVar3 + 0x18))) ||
        (bVar1 = false, local_68 != *(ulonglong *)(lVar3 + 0x20))))) {
      bVar1 = true;
    }
    iVar8 = 0;
    iVar4 = -0x4520531d;
    if (!bVar1) {
      lVar3 = *(longlong *)(local_a0 + 0x20);
      if (((local_60 != *(ulonglong *)(lVar3 + 8)) || (local_58 != *(ulonglong *)(lVar3 + 0x10))) ||
         ((local_50 != *(ulonglong *)(lVar3 + 0x18) ||
          (bVar1 = false, local_48 != *(ulonglong *)(lVar3 + 0x20))))) {
        bVar1 = true;
      }
      if (!bVar1) {
        local_28 = *(ulonglong *)(unaff_R7 + 0x210) ^ 0x4a0178651b8c3c5;
        local_30 = *(ulonglong *)(unaff_R7 + 0x208) ^ 0x4a1178751b9c3c6;
        local_38 = *(ulonglong *)(unaff_R7 + 0x200) ^ 0x4a2178451bac3c7;
        local_40 = *(ulonglong *)(unaff_R7 + 0x1f8) ^ 0xfb5ce87aae443c38;
        local_8 = *(ulonglong *)(unaff_R7 + 0x1f0) ^ 0x4a0178651b8c3c5;
        local_10 = *(ulonglong *)(unaff_R7 + 0x1e8) ^ 0x4a1178751b9c3c6;
        local_18 = *(ulonglong *)(unaff_R7 + 0x1e0) ^ 0x4a2178451bac3c7;
        local_20 = *(ulonglong *)(unaff_R7 + 0x1d8) ^ 0xfb5ce87aae443c38;
        if (((*unaff_R8 == 0) ||
            (FUN_ram_0000d038(&local_88,(longlong *)(local_a0 + 0x10),local_a0 + 0x18,local_a0 + 8),
            iVar4 = local_84, iVar8 = local_88, local_88 == 0x1a)) &&
           ((iVar5 = (int)(local_a0 + 8), unaff_R8[1] == 0 ||
            (FUN_ram_0000d038(&local_90,(longlong *)(local_a0 + 0x20),local_a0 + 0x28),
            iVar4 = local_8c, iVar8 = local_90, local_90 == 0x1a)))) {
          *(ulonglong *)(unaff_R7 + 0x2a8) = unaff_R8[2] ^ 0x6e9de2b30b19f9ea;
          iVar4 = iVar5;
          iVar8 = 0x1a;
        }
      }
    }
  }
  unaff_R9[1] = iVar4;
  *unaff_R9 = iVar8;
  return;
}

// Function: FUN_ram_00012580
void FUN_ram_00012580(undefined4 *param_1,longlong *param_2,longlong param_3,undefined2 *param_4,
                     longlong param_5)

{
  bool bVar1;
  undefined4 uVar2;
  longlong lVar3;
  undefined4 uVar4;
  longlong lVar5;
  ulonglong local_20;
  ulonglong local_18;
  ulonglong local_10;
  ulonglong local_8;
  
  uVar2 = 0;
  uVar4 = 0xbadc0de;
  if (param_3 == 2) {
    lVar3 = param_2[1];
    lVar5 = *param_2;
    local_8 = *(ulonglong *)(lVar3 + 0x270);
    local_10 = *(ulonglong *)(lVar3 + 0x268);
    local_18 = *(ulonglong *)(lVar3 + 0x260);
    local_20 = *(ulonglong *)(lVar3 + 600);
    if (*(ulonglong *)(lVar3 + 0x710) < 5) {
      local_18 = local_18 ^ 0x4a2178451bac3c7;
      local_20 = local_20 ^ 0xfb5ce87aae443c38;
      local_10 = local_10 ^ 0x4a1178751b9c3c6;
      local_8 = local_8 ^ 0x4a0178651b8c3c5;
    }
    if ((((local_20 != *(ulonglong *)(lVar5 + 8)) || (local_18 != *(ulonglong *)(lVar5 + 0x10))) ||
        (local_10 != *(ulonglong *)(lVar5 + 0x18))) ||
       (bVar1 = false, local_8 != *(ulonglong *)(lVar5 + 0x20))) {
      bVar1 = true;
    }
    uVar4 = 0xabad1dea;
    if ((*(char *)(lVar5 + 1) != '\0') && (!bVar1)) {
      if (param_5 != 8) {
                    /* WARNING: Subroutine does not return */
        FUN_ram_000011b0(&DAT_ram_000337df,10,2);
      }
      if (((ulonglong)param_4 & 1) != 0) {
                    /* WARNING: Subroutine does not return */
        FUN_ram_000011b0(&DAT_ram_000337df,10,0);
      }
      *(undefined2 *)(lVar3 + 0x318) = *param_4;
      *(undefined4 *)(lVar3 + 0x31a) = 0;
      *(undefined2 *)(lVar3 + 0x31e) = 0;
      *(ulonglong *)(lVar3 + 0x318) = *(ulonglong *)(lVar3 + 0x318) ^ 0xed5f563e78eee80b;
      uVar2 = 0x1a;
      if (*(ulonglong *)(lVar3 + 0x710) == 0) {
        *(undefined8 *)(lVar3 + 0x710) = 1;
      }
    }
  }
  param_1[1] = uVar4;
  *param_1 = uVar2;
  return;
}

// Function: FUN_ram_00012848
undefined8
FUN_ram_00012848(longlong param_1,undefined8 *param_2,undefined8 *param_3,ulonglong param_4,
                longlong param_5)

{
  ushort uVar1;
  bool bVar2;
  ushort *puVar3;
  ulonglong uVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  ulonglong local_38;
  longlong alStack_30 [4];
  undefined8 *local_10;
  undefined8 local_8;
  
  local_80 = 0x2a9f6e5d1c8b4a2f;
  local_88 = 0x9e6d5c1b8a4f2e9d;
  local_90 = 0x6c5b1a8f4e2d9c6b;
  local_98 = 0x5a1f8e4d2c9b3a7f;
  local_78 = *param_2;
  local_70 = param_2[1];
  local_68 = param_2[2];
  local_60 = param_2[3];
  local_58 = *param_3;
  local_50 = param_3[1];
  local_48 = param_3[2];
  local_40 = param_3[3];
  local_10 = &local_98;
  local_8 = 0x68;
  local_38 = param_4 >> 7;
  FUN_ram_000129b0(&local_10,1,alStack_30);
  if (alStack_30[0] == param_1) {
    return 1;
  }
  lVar5 = *(longlong *)(param_5 + -0xff8);
  puVar3 = *(ushort **)(param_5 + -0x1000);
  local_80 = 0x2a9f6e5d1c8b4a2f;
  local_88 = 0x9e6d5c1b8a4f2e9d;
  local_90 = 0x6c5b1a8f4e2d9c6b;
  local_98 = 0x5a1f8e4d2c9b3a7f;
  local_78 = *param_2;
  local_70 = param_2[1];
  local_68 = param_2[2];
  local_60 = param_2[3];
  local_58 = *param_3;
  local_50 = param_3[1];
  local_48 = param_3[2];
  local_40 = param_3[3];
  local_38 = (param_4 >> 7) - 1;
  local_10 = &local_98;
  local_8 = 0x68;
  FUN_ram_00012b30(&local_10,1,alStack_30);
  if (alStack_30[0] == param_1) {
    return 1;
  }
  uVar4 = (ulonglong)*(ushort *)((longlong)puVar3 + lVar5 + -2);
  if (uVar4 < *puVar3) {
    puVar3 = (ushort *)((longlong)puVar3 + (ulonglong)puVar3[uVar4 + 1]);
    uVar1 = *puVar3;
    if ((((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 2) != 0x6ec031f25bd57904) ||
         (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) != 0x71568ce6ec574ee)) ||
        (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x518ef4a3deb2b1fd)) ||
       (bVar2 = false,
       *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) != -0x70ec43a95d324efe)) {
      bVar2 = true;
    }
    if (bVar2) {
      if (((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 2) != -0x3b66289859b23cf6) ||
          (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) != 0x75b1926ae1365115)) ||
         ((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x678ad2090231d088 ||
          (bVar2 = false,
          *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) != -0x139993aed94b961d)))
         ) {
        bVar2 = true;
      }
      if (bVar2) {
        if (((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 2) != 0x715b8f7af9be1205)
            || (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                -0x3fbd123929120c83)) ||
           ((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) != -0x1178411a20edb01e
            || (bVar2 = false,
               *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
               -0x4693a2c08ba113c1)))) {
          bVar2 = true;
        }
        if (bVar2) goto LAB_ram_00012d98;
      }
    }
    local_80 = 0x2a9f6e5d1c8b4a2f;
    local_88 = 0x9e6d5c1b8a4f2e9d;
    local_90 = 0x6c5b1a8f4e2d9c6b;
    local_98 = 0x5a1f8e4d2c9b3a7f;
    local_78 = *param_2;
    local_70 = param_2[1];
    local_68 = param_2[2];
    local_60 = param_2[3];
    local_38 = 0;
    local_58 = *param_3;
    local_50 = param_3[1];
    local_48 = param_3[2];
    local_40 = param_3[3];
    local_10 = &local_98;
    local_8 = 0x68;
    uVar6 = 0;
    FUN_ram_00012ee0(&local_10,1,alStack_30);
    if (alStack_30[0] == param_1) {
      uVar6 = 1;
    }
  }
  else {
LAB_ram_00012d98:
    uVar6 = 0;
  }
  return uVar6;
}

// Function: FUN_ram_000129b0
undefined8 FUN_ram_000129b0(void)

{
  ushort uVar1;
  bool bVar2;
  ushort *puVar3;
  ulonglong uVar4;
  longlong lVar5;
  longlong unaff_R6;
  undefined8 *unaff_R7;
  undefined8 *unaff_R8;
  undefined8 unaff_R9;
  undefined8 uVar6;
  longlong local_a8;
  longlong local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  longlong local_38;
  longlong local_30 [4];
  undefined8 *local_10;
  undefined8 local_8;
  
  FUN_ram_000129b0();
  if (local_30[0] == local_a0) {
    return unaff_R9;
  }
  lVar5 = *(longlong *)(local_a8 + -0xff8);
  puVar3 = *(ushort **)(local_a8 + -0x1000);
  local_80 = 0x2a9f6e5d1c8b4a2f;
  local_88 = 0x9e6d5c1b8a4f2e9d;
  local_90 = 0x6c5b1a8f4e2d9c6b;
  local_98 = 0x5a1f8e4d2c9b3a7f;
  local_78 = *unaff_R7;
  local_70 = unaff_R7[1];
  local_68 = unaff_R7[2];
  local_60 = unaff_R7[3];
  local_58 = *unaff_R8;
  local_50 = unaff_R8[1];
  local_48 = unaff_R8[2];
  local_40 = unaff_R8[3];
  local_38 = unaff_R6 + -1;
  local_10 = &local_98;
  local_8 = 0x68;
  FUN_ram_00012b30(&local_10,1,local_30);
  if (local_30[0] == local_a0) {
    return unaff_R9;
  }
  uVar4 = (ulonglong)*(ushort *)((longlong)puVar3 + lVar5 + -2);
  if (uVar4 < *puVar3) {
    puVar3 = (ushort *)((longlong)puVar3 + (ulonglong)puVar3[uVar4 + 1]);
    uVar1 = *puVar3;
    if ((((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 2) != 0x6ec031f25bd57904) ||
         (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) != 0x71568ce6ec574ee)) ||
        (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x518ef4a3deb2b1fd)) ||
       (bVar2 = false,
       *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) != -0x70ec43a95d324efe)) {
      bVar2 = true;
    }
    if (bVar2) {
      if (((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 2) != -0x3b66289859b23cf6) ||
          (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) != 0x75b1926ae1365115)) ||
         ((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x678ad2090231d088 ||
          (bVar2 = false,
          *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) != -0x139993aed94b961d)))
         ) {
        bVar2 = true;
      }
      if (bVar2) {
        if (((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 2) != 0x715b8f7af9be1205)
            || (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                -0x3fbd123929120c83)) ||
           ((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) != -0x1178411a20edb01e
            || (bVar2 = false,
               *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
               -0x4693a2c08ba113c1)))) {
          bVar2 = true;
        }
        if (bVar2) goto LAB_ram_00012d98;
      }
    }
    local_80 = 0x2a9f6e5d1c8b4a2f;
    local_88 = 0x9e6d5c1b8a4f2e9d;
    local_90 = 0x6c5b1a8f4e2d9c6b;
    local_98 = 0x5a1f8e4d2c9b3a7f;
    local_78 = *unaff_R7;
    local_70 = unaff_R7[1];
    local_68 = unaff_R7[2];
    local_60 = unaff_R7[3];
    local_38 = 0;
    local_58 = *unaff_R8;
    local_50 = unaff_R8[1];
    local_48 = unaff_R8[2];
    local_40 = unaff_R8[3];
    local_10 = &local_98;
    local_8 = 0x68;
    uVar6 = 0;
    FUN_ram_00012ee0(&local_10,1,local_30);
    if (local_30[0] == local_a0) {
      uVar6 = 1;
    }
  }
  else {
LAB_ram_00012d98:
    uVar6 = 0;
  }
  return uVar6;
}

// Function: FUN_ram_00012b30
undefined8 FUN_ram_00012b30(void)

{
  ushort uVar1;
  bool bVar2;
  ulonglong uVar3;
  ushort *puVar4;
  undefined8 *unaff_R7;
  undefined8 *unaff_R8;
  undefined8 unaff_R9;
  undefined8 uVar5;
  longlong local_b0;
  ushort *local_a8;
  longlong local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined8 local_38;
  longlong local_30 [4];
  undefined8 *local_10;
  undefined8 local_8;
  
  FUN_ram_00012b30();
  if (local_30[0] == local_a0) {
    return unaff_R9;
  }
  uVar3 = (ulonglong)*(ushort *)((longlong)local_a8 + local_b0 + -2);
  if (uVar3 < *local_a8) {
    puVar4 = (ushort *)((longlong)local_a8 + (ulonglong)local_a8[uVar3 + 1]);
    uVar1 = *puVar4;
    if ((((*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 2) != 0x6ec031f25bd57904) ||
         (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) != 0x71568ce6ec574ee)) ||
        (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x518ef4a3deb2b1fd)) ||
       (bVar2 = false,
       *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) != -0x70ec43a95d324efe)) {
      bVar2 = true;
    }
    if (bVar2) {
      if (((*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 2) != -0x3b66289859b23cf6) ||
          (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) != 0x75b1926ae1365115)) ||
         ((*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x678ad2090231d088 ||
          (bVar2 = false,
          *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) != -0x139993aed94b961d)))
         ) {
        bVar2 = true;
      }
      if (bVar2) {
        if (((*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 2) != 0x715b8f7af9be1205)
            || (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                -0x3fbd123929120c83)) ||
           ((*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) != -0x1178411a20edb01e
            || (bVar2 = false,
               *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
               -0x4693a2c08ba113c1)))) {
          bVar2 = true;
        }
        if (bVar2) goto LAB_ram_00012d98;
      }
    }
    local_80 = 0x2a9f6e5d1c8b4a2f;
    local_88 = 0x9e6d5c1b8a4f2e9d;
    local_90 = 0x6c5b1a8f4e2d9c6b;
    local_98 = 0x5a1f8e4d2c9b3a7f;
    local_78 = *unaff_R7;
    local_70 = unaff_R7[1];
    local_68 = unaff_R7[2];
    local_60 = unaff_R7[3];
    local_38 = 0;
    local_58 = *unaff_R8;
    local_50 = unaff_R8[1];
    local_48 = unaff_R8[2];
    local_40 = unaff_R8[3];
    local_10 = &local_98;
    local_8 = 0x68;
    uVar5 = 0;
    FUN_ram_00012ee0(&local_10,1,local_30);
    if (local_30[0] == local_a0) {
      uVar5 = 1;
    }
  }
  else {
LAB_ram_00012d98:
    uVar5 = 0;
  }
  return uVar5;
}

// Function: FUN_ram_00012ee0
undefined8 FUN_ram_00012ee0(void)

{
  longlong unaff_R6;
  undefined8 unaff_R9;
  undefined8 local_30;
  
  FUN_ram_00012ee0();
  if (local_30 == unaff_R6) {
    unaff_R9 = 1;
  }
  return unaff_R9;
}

// Function: FUN_ram_00012f10
void FUN_ram_00012f10(int *param_1,longlong *param_2,longlong param_3,ulonglong param_4,
                     longlong param_5)

{
  bool bVar1;
  ulonglong uVar2;
  longlong lVar3;
  int iVar4;
  ulonglong uVar5;
  longlong lVar6;
  int local_28 [2];
  ulonglong local_20;
  ulonglong local_18;
  ulonglong local_10;
  ulonglong local_8;
  
  iVar4 = 0;
  uVar2 = 0xbadc0de;
  if (param_3 == 2) {
    lVar6 = param_2[1];
    lVar3 = *param_2;
    local_8 = *(ulonglong *)(lVar6 + 0x270);
    local_10 = *(ulonglong *)(lVar6 + 0x268);
    local_18 = *(ulonglong *)(lVar6 + 0x260);
    local_20 = *(ulonglong *)(lVar6 + 600);
    uVar5 = *(ulonglong *)(lVar6 + 0x710);
    if (uVar5 < 5) {
      local_18 = local_18 ^ 0x4a2178451bac3c7;
      local_20 = local_20 ^ 0xfb5ce87aae443c38;
      local_10 = local_10 ^ 0x4a1178751b9c3c6;
      local_8 = local_8 ^ 0x4a0178651b8c3c5;
    }
    if ((((local_20 != *(ulonglong *)(lVar3 + 8)) || (local_18 != *(ulonglong *)(lVar3 + 0x10))) ||
        (local_10 != *(ulonglong *)(lVar3 + 0x18))) ||
       (bVar1 = false, local_8 != *(ulonglong *)(lVar3 + 0x20))) {
      bVar1 = true;
    }
    uVar2 = 0xabad1dea;
    if ((*(char *)(lVar3 + 1) != '\0') && (!bVar1)) {
      if (uVar5 == 1) {
        *(undefined8 *)(lVar6 + 0x710) = 2;
      }
      else {
        uVar2 = 0xbad4;
        if (uVar5 == 0) goto LAB_ram_00013350;
      }
      if (param_5 != 0x50) {
                    /* WARNING: Subroutine does not return */
        FUN_ram_000011b0(&DAT_ram_000337df,10,2);
      }
      if ((param_4 & 3) != 0) {
                    /* WARNING: Subroutine does not return */
        FUN_ram_000011b0(&DAT_ram_000337df,10,0);
      }
      FUN_ram_00016a10(local_28,param_4);
      uVar2 = 0xbad3;
      iVar4 = local_28[0];
      if (local_28[0] == 0x1a) {
        FUN_ram_00031b28(lVar6 + 0x328,param_4,0x50);
        uVar2 = *(ulonglong *)(lVar6 + 0x330) ^ 0x47d26c2e77aa1400;
        *(ulonglong *)(lVar6 + 0x328) = *(ulonglong *)(lVar6 + 0x328) ^ 0xb82c93d08854ebff;
        *(ulonglong *)(lVar6 + 0x330) = uVar2;
        *(ulonglong *)(lVar6 + 0x338) = *(ulonglong *)(lVar6 + 0x338) ^ 0x47d16c2d77a91401;
        *(ulonglong *)(lVar6 + 0x340) = *(ulonglong *)(lVar6 + 0x340) ^ 0x47d06c2c77a81402;
        *(ulonglong *)(lVar6 + 0x348) = *(ulonglong *)(lVar6 + 0x348) ^ 0x47d76c2b77af1403;
        *(ulonglong *)(lVar6 + 0x350) = *(ulonglong *)(lVar6 + 0x350) ^ 0x47d66c2a77ae1404;
        *(ulonglong *)(lVar6 + 0x358) = *(ulonglong *)(lVar6 + 0x358) ^ 0x47d56c2977ad1405;
        *(ulonglong *)(lVar6 + 0x360) = *(ulonglong *)(lVar6 + 0x360) ^ 0x47d46c2877ac1406;
        *(ulonglong *)(lVar6 + 0x368) = *(ulonglong *)(lVar6 + 0x368) ^ 0x47db6c2777a31407;
        *(ulonglong *)(lVar6 + 0x370) = *(ulonglong *)(lVar6 + 0x370) ^ 0x47da6c2677a21408;
        iVar4 = 0x1a;
      }
    }
  }
LAB_ram_00013350:
  param_1[1] = (int)uVar2;
  *param_1 = iVar4;
  return;
}

// Function: FUN_ram_000133b8
ulonglong FUN_ram_000133b8(undefined8 param_1)

{
  byte bVar1;
  bool bVar2;
  uint *puVar3;
  ulonglong uVar4;
  longlong *plVar5;
  ulonglong uVar6;
  undefined1 uVar7;
  longlong lVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  undefined1 uVar11;
  char *pcVar12;
  longlong lVar13;
  char *pcVar14;
  undefined1 *puVar15;
  ulonglong uVar16;
  undefined8 uVar17;
  uint local_3b0 [6];
  char acStack_398 [16];
  undefined4 uStack_388;
  char acStack_375 [40];
  undefined1 auStack_34d [809];
  char local_24 [34];
  char local_2;
  char local_1;
  
  FUN_ram_00013658(local_3b0,param_1);
  if ((ulonglong)local_3b0[0] != 0) {
    lVar8 = 0;
    uVar4 = 0;
    do {
      if (acStack_375[lVar8] == '\x17') {
        local_1 = acStack_398[lVar8 + 1];
        local_2 = acStack_398[lVar8];
        if (0xf < (uVar4 & 0xffffffff)) {
          puVar3 = (uint *)&DAT_ram_00033764;
          plVar5 = (longlong *)0x2b;
          uVar7 = 0x70;
          uVar11 = 0xb0;
          uVar4 = FUN_ram_0002fd08(&DAT_ram_00033764,0x2b,&local_2);
          bVar1 = *(byte *)((longlong)plVar5 + 0x14);
          if (1 < bVar1) {
            if (bVar1 != 2) {
              *puVar3 = 0;
              return uVar4;
            }
            uVar10 = plVar5[1];
            bVar2 = 0x15 < uVar10;
            if (bVar2) {
              lVar8 = *plVar5;
              uVar7 = *(undefined1 *)(lVar8 + 0x15);
              uVar11 = *(undefined1 *)(lVar8 + 0xc);
              uVar10 = *(ulonglong *)(lVar8 + 0xd);
            }
            *(undefined1 *)((longlong)puVar3 + 0x3b) = uVar11;
            *(undefined1 *)((longlong)puVar3 + 0x19) = uVar7;
            *(ulonglong *)(puVar3 + 4) = uVar10;
            *puVar3 = (uint)bVar2;
            *(undefined1 *)((longlong)puVar3 + 0x1a) = 0;
            *(undefined1 *)(puVar3 + 6) = 1;
            puVar3[2] = 1;
            puVar3[3] = 0;
            return uVar4;
          }
          if ((bVar1 == 0) || (uVar4 = plVar5[1], 0x3b < uVar4)) {
            uVar4 = FUN_ram_00013838();
            return uVar4;
          }
          uVar10 = (ulonglong)*(uint *)(plVar5 + 2);
          if (uVar10 != 0) {
            lVar8 = *plVar5;
            uVar6 = 0;
            uVar16 = 0x16;
            puVar15 = auStack_34d;
            goto LAB_ram_00013700;
          }
          uVar9 = 0;
          goto LAB_ram_00013808;
        }
        lVar13 = (uVar4 & 0xffffffff) * 2;
        local_24[lVar13 + 1] = local_1;
        local_24[lVar13] = local_2;
        uVar4 = uVar4 + 1;
      }
      lVar8 = lVar8 + 0x38;
    } while ((ulonglong)local_3b0[0] * 0x38 != lVar8);
    uVar4 = uVar4 & 0xffffffff;
    if (uVar4 < 2) {
      return 0;
    }
    pcVar12 = local_24 + 3;
    uVar10 = 0;
    do {
      uVar9 = uVar10 + 1;
      if (uVar4 <= uVar9) {
        return 0;
      }
      if (local_24[uVar10 * 2] == '\x01') {
        pcVar14 = pcVar12;
        uVar16 = uVar9;
        do {
          if ((pcVar14[-1] == '\x01') && (local_24[uVar10 * 2 + 1] != *pcVar14)) {
            return (ulonglong)(1 < uVar4);
          }
          pcVar14 = pcVar14 + 2;
          uVar16 = uVar16 + 1;
        } while (uVar16 < uVar4);
      }
      pcVar12 = pcVar12 + 2;
      uVar10 = uVar9;
    } while (uVar9 < uVar4);
  }
  return 0;
  while( true ) {
    lVar13 = lVar8 + uVar16;
    uVar17 = *(undefined8 *)(lVar13 + -9);
    uVar7 = *(undefined1 *)(lVar13 + -1);
    *puVar15 = *(undefined1 *)(lVar13 + -10);
    puVar15[-0x22] = uVar7;
    *(undefined8 *)(puVar15 + -0x2b) = uVar17;
    uVar16 = uVar16 + 0xb;
    puVar15[-0x21] = 0;
    puVar15[-0x23] = 1;
    *(undefined8 *)(puVar15 + -0x33) = 1;
    puVar15 = puVar15 + 0x38;
    uVar6 = uVar6 + 1;
    uVar9 = uVar10;
    if (uVar10 <= uVar6) break;
LAB_ram_00013700:
    uVar9 = uVar6;
    if ((uVar16 == 0xc6) || (uVar4 < uVar16)) break;
  }
LAB_ram_00013808:
  uStack_388 = (undefined4)uVar9;
  uVar4 = FUN_ram_00031b28();
  return uVar4;
}

// Function: FUN_ram_00013658
void FUN_ram_00013658(uint *param_1,longlong *param_2,undefined8 param_3,undefined1 param_4,
                     undefined1 param_5)

{
  byte bVar1;
  undefined1 uVar2;
  bool bVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  longlong lVar9;
  undefined1 *puVar10;
  longlong lVar11;
  undefined8 uVar12;
  undefined4 local_388 [14];
  undefined1 local_34d [845];
  
  bVar1 = *(byte *)((longlong)param_2 + 0x14);
  if (bVar1 < 2) {
    if ((bVar1 == 0) || (uVar7 = param_2[1], 0x3b < uVar7)) {
      FUN_ram_00013838();
    }
    else {
      uVar8 = (ulonglong)*(uint *)(param_2 + 2);
      if (uVar8 == 0) {
        uVar6 = 0;
      }
      else {
        lVar9 = *param_2;
        uVar5 = 0;
        uVar4 = 0x16;
        puVar10 = local_34d;
        do {
          uVar6 = uVar5;
          if ((uVar4 == 0xc6) || (uVar7 < uVar4)) break;
          lVar11 = lVar9 + uVar4;
          uVar12 = *(undefined8 *)(lVar11 + -9);
          uVar2 = *(undefined1 *)(lVar11 + -1);
          *puVar10 = *(undefined1 *)(lVar11 + -10);
          puVar10[-0x22] = uVar2;
          *(undefined8 *)(puVar10 + -0x2b) = uVar12;
          uVar4 = uVar4 + 0xb;
          puVar10[-0x21] = 0;
          puVar10[-0x23] = 1;
          *(undefined8 *)(puVar10 + -0x33) = 1;
          puVar10 = puVar10 + 0x38;
          uVar5 = uVar5 + 1;
          uVar6 = uVar8;
        } while (uVar5 < uVar8);
      }
      local_388[0] = (undefined4)uVar6;
      FUN_ram_00031b28(param_1,local_388,0x388);
    }
  }
  else if (bVar1 == 2) {
    uVar7 = param_2[1];
    bVar3 = 0x15 < uVar7;
    if (bVar3) {
      lVar9 = *param_2;
      param_4 = *(undefined1 *)(lVar9 + 0x15);
      param_5 = *(undefined1 *)(lVar9 + 0xc);
      uVar7 = *(ulonglong *)(lVar9 + 0xd);
    }
    *(undefined1 *)((longlong)param_1 + 0x3b) = param_5;
    *(undefined1 *)((longlong)param_1 + 0x19) = param_4;
    *(ulonglong *)(param_1 + 4) = uVar7;
    *param_1 = (uint)bVar3;
    *(undefined1 *)((longlong)param_1 + 0x1a) = 0;
    *(undefined1 *)(param_1 + 6) = 1;
    param_1[2] = 1;
    param_1[3] = 0;
  }
  else {
    *param_1 = 0;
  }
  return;
}

// Function: FUN_ram_00013838
void FUN_ram_00013838(undefined8 param_1,longlong *param_2)

{
  undefined1 uVar1;
  bool bVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  longlong lVar8;
  bool bVar9;
  undefined1 *puVar10;
  longlong lVar11;
  longlong lVar12;
  byte *pbVar13;
  undefined4 local_388 [6];
  byte abStack_370 [35];
  undefined1 local_34d [845];
  
  uVar5 = param_2[1];
  lVar8 = *param_2;
  uVar6 = (ulonglong)*(uint *)(param_2 + 2);
  uVar3 = 0;
  uVar4 = uVar3;
  if (uVar6 != 0) {
    uVar7 = 0x2d;
    puVar10 = local_34d;
    do {
      uVar4 = uVar3;
      if ((uVar5 < uVar7) || (uVar7 == 0x23d)) break;
      lVar12 = lVar8 + uVar7;
      uVar1 = *(undefined1 *)(lVar12 + -0x21);
      puVar10[-0x21] = 1;
      puVar10[-0x23] = 0;
      *(undefined8 *)(puVar10 + -0x33) = 0;
      *(undefined8 *)(puVar10 + -0x20) = *(undefined8 *)(lVar12 + -0x20);
      *(undefined8 *)(puVar10 + -0x18) = *(undefined8 *)(lVar12 + -0x18);
      *(undefined8 *)(puVar10 + -0x10) = *(undefined8 *)(lVar12 + -0x10);
      *(undefined8 *)(puVar10 + -8) = *(undefined8 *)(lVar12 + -8);
      *puVar10 = uVar1;
      uVar7 = uVar7 + 0x21;
      puVar10 = puVar10 + 0x38;
      uVar3 = uVar3 + 1;
      uVar4 = uVar6;
    } while (uVar3 < uVar6);
  }
  uVar3 = uVar5 - 4;
  local_388[0] = (undefined4)uVar4;
  if ((uVar3 <= uVar5) && (0xc < uVar3)) {
    lVar11 = lVar8 + 0xf;
    lVar12 = 0;
    do {
      if (uVar5 <= lVar12 + 0xcU) {
                    /* WARNING: Subroutine does not return */
        FUN_ram_0002fc40(lVar12 + 0xcU,uVar5,&DAT_ram_000342e0);
      }
      if (*(char *)(lVar11 + lVar12 + -3) == '\0') {
        if (uVar5 <= lVar12 + 0xdU) {
                    /* WARNING: Subroutine does not return */
          FUN_ram_0002fc40(lVar12 + 0xd,uVar5,&DAT_ram_000342f8);
        }
        if (*(char *)(lVar11 + lVar12 + -2) == '\0') {
          if (uVar5 <= lVar12 + 0xeU) {
                    /* WARNING: Subroutine does not return */
            FUN_ram_0002fc40(lVar12 + 0xeU,uVar5,&DAT_ram_00034310);
          }
          if (*(char *)(lVar11 + lVar12 + -1) == '\0') {
            if (uVar5 <= lVar12 + 0xfU) {
                    /* WARNING: Subroutine does not return */
              FUN_ram_0002fc40(lVar12 + 0xfU,uVar5,&DAT_ram_00034328);
            }
            uVar6 = (ulonglong)*(byte *)(lVar11 + lVar12);
            if ((((uVar6 < 0x24) && ((1L << uVar6 & 0x800101017U) != 0)) || (uVar6 == 0x81)) &&
               (lVar12 + 0x10U < uVar5)) {
              uVar3 = lVar12 + 0xf;
              lVar8 = uVar3 + lVar8;
              goto code_r0x00013dc8;
            }
          }
        }
      }
      uVar6 = lVar12 + 0xd;
      lVar12 = lVar12 + 1;
    } while (uVar6 < uVar3);
  }
  if (3 < uVar5) {
    lVar12 = -4;
    lVar8 = lVar8 + 4;
    do {
      if (*(int *)(lVar8 + -4) == 0x78) {
        uVar3 = -lVar12;
        if (uVar3 < uVar5) {
code_r0x00013dc8:
          bVar2 = uVar5 != uVar3;
          lVar12 = 0;
          uVar5 = uVar5 - uVar3;
          uVar3 = uVar4 & 0xffffffff;
          bVar9 = uVar3 != 0;
          if ((uVar5 < 9) || ((uVar4 & 0xffffffff) == 0)) {
            uVar4 = 0;
          }
          else {
            pbVar13 = abStack_370 + 1;
            uVar4 = 0;
            lVar11 = 0;
            do {
              if (!bVar2) {
                    /* WARNING: Subroutine does not return */
                FUN_ram_0002fc40(lVar11,uVar5,&DAT_ram_000342c8);
              }
              bVar2 = lVar11 + 9U < uVar5;
              uVar4 = uVar4 + 1;
              bVar9 = uVar4 < uVar3;
              *pbVar13 = *(byte *)(lVar8 + lVar11) & 1;
              pbVar13[-1] = 1;
              lVar12 = lVar11 + 9;
            } while ((lVar11 + 0x12U <= uVar5) &&
                    (pbVar13 = pbVar13 + 0x38, lVar11 = lVar12, uVar4 < uVar3));
          }
          if ((bVar2) && (bVar9)) {
            abStack_370[uVar4 * 0x38 + 1] = *(byte *)(lVar8 + lVar12) & 1;
            abStack_370[uVar4 * 0x38] = 1;
          }
        }
        break;
      }
      lVar8 = lVar8 + 1;
      lVar12 = lVar12 + -1;
    } while (4 < uVar5 + lVar12 + 5);
  }
  FUN_ram_00031b28(param_1,local_388,0x388);
  return;
}

// Function: FUN_ram_00013e80
void FUN_ram_00013e80(undefined1 *param_1,byte *param_2,undefined *param_3,undefined *param_4)

{
  byte bVar1;
  uint uVar2;
  undefined *puVar3;
  undefined1 uVar4;
  undefined *puVar5;
  ulonglong uVar6;
  undefined *puVar7;
  ulonglong uVar8;
  undefined *puVar9;
  
  if (param_3 <= param_4) goto LAB_ram_000140e0;
  uVar2 = 1;
  bVar1 = param_2[(longlong)param_4];
  uVar8 = (ulonglong)bVar1;
  if (uVar8 < 0x4f) {
    if (uVar8 < 10) {
      if (uVar8 < 8) {
        puVar9 = (undefined *)0x1;
        if ((1L << uVar8 & 0x89U) != 0) goto LAB_ram_000140a8;
      }
      else {
        puVar9 = (undefined *)0x1;
      }
      goto LAB_ram_000140b8;
    }
    uVar6 = uVar8 - 10;
    if (uVar6 < 0x3e) {
      if ((1L << uVar6 & 0x200405050210201U) != 0) goto LAB_ram_000140a8;
      if ((1L << uVar6 & 0x2000002000000000U) != 0) {
LAB_ram_00014018:
        puVar9 = (undefined *)0x2;
        goto LAB_ram_000140b8;
      }
      if (uVar6 != 0x1f) goto LAB_ram_00013f30;
    }
    else {
LAB_ram_00013f30:
      if (uVar8 != 0x4b) {
        puVar9 = (undefined *)0x1;
        if (uVar8 == 0x4d) goto LAB_ram_000140a8;
        goto LAB_ram_000140b8;
      }
    }
    puVar9 = (undefined *)0x4;
  }
  else if (uVar8 < 0x66) {
    if (uVar8 < 0x57) {
      if (uVar8 - 0x51 < 2) goto LAB_ram_00014078;
      if (uVar8 == 0x4f) goto LAB_ram_000140a8;
      puVar9 = (undefined *)0x1;
      if (uVar8 == 0x56) goto LAB_ram_00014018;
    }
    else if (uVar8 < 99) {
      if (uVar8 == 0x57) {
LAB_ram_00013fd8:
        puVar9 = (undefined *)0x9;
      }
      else if ((uVar8 == 0x58) || (puVar9 = (undefined *)0x1, uVar8 == 0x60)) goto LAB_ram_000140a8;
    }
    else {
      puVar9 = (undefined *)0x1;
      if (uVar8 - 99 < 2) {
LAB_ram_000140a8:
        uVar2 = 0;
        puVar9 = (undefined *)0x0;
      }
    }
  }
  else {
    uVar6 = uVar8 - 0x69;
    if (uVar6 < 0xe) {
      if ((1L << uVar6 & 0x189U) != 0) goto LAB_ram_000140a8;
      if (uVar6 == 6) {
        FUN_ram_000144a0();
        return;
      }
      if (uVar6 == 0xd) goto LAB_ram_00013fd8;
    }
    if (uVar8 == 0x66) goto LAB_ram_000140a8;
    puVar9 = (undefined *)0x1;
    if (uVar8 != 0x67) goto LAB_ram_000140b8;
LAB_ram_00014078:
    puVar9 = (undefined *)0x8;
  }
LAB_ram_000140b8:
  puVar7 = param_4 + 1;
  if (param_3 < puVar9 + (longlong)puVar7) {
LAB_ram_000140e0:
    *param_1 = 7;
    return;
  }
  if (puVar9 < (undefined *)0x2) {
    if (puVar9 == (undefined *)0x0) {
      uVar4 = 0;
      goto LAB_ram_000142c0;
    }
    if (puVar9 == (undefined *)0x1) {
      if (puVar7 < param_3) {
        param_2 = (byte *)(ulonglong)param_2[(longlong)puVar7];
        uVar4 = 1;
        uVar2 = (uint)(param_2 != (byte *)0x0);
        goto LAB_ram_000142c0;
      }
      goto LAB_ram_00014428;
    }
LAB_ram_00014290:
    if (uVar2 != 0) {
      if (param_3 <= puVar7) {
                    /* WARNING: Subroutine does not return */
        FUN_ram_0002fc40(puVar7,param_3,&DAT_ram_000343b8);
      }
      param_2 = param_2 + (longlong)puVar7;
      param_4 = (undefined *)(ulonglong)*param_2;
    }
    uVar4 = 6;
    puVar7 = puVar9;
LAB_ram_000142c0:
    *(undefined **)(param_1 + 8) = puVar7;
    *(int *)(param_1 + 4) = (int)param_2;
    param_1[3] = (char)param_4;
    param_1[2] = (char)uVar2;
    param_1[1] = bVar1;
    *param_1 = uVar4;
    *(undefined **)(param_1 + 0x10) = puVar9 + 1;
    return;
  }
  if (puVar9 == (undefined *)0x2) {
    puVar3 = param_4 + 3;
    if (param_4 < (undefined *)0xfffffffffffffffd) {
      puVar5 = param_3;
      if (puVar3 <= param_3) {
        param_2 = param_2 + (longlong)puVar7;
        uVar4 = 2;
        param_4 = (undefined *)(ulonglong)param_2[1];
        uVar2 = (uint)*param_2;
        goto LAB_ram_000142c0;
      }
    }
    else {
      puVar3 = (undefined *)FUN_ram_00031048(puVar7,puVar3,&DAT_ram_00034358);
LAB_ram_00014360:
      puVar3 = (undefined *)FUN_ram_00031048(puVar7,puVar3,&DAT_ram_00034370);
LAB_ram_00014388:
      puVar5 = &DAT_ram_00034388;
      puVar3 = (undefined *)FUN_ram_00031048(puVar7,puVar3,&DAT_ram_00034388);
    }
    param_3 = &DAT_ram_00034358;
    puVar3 = (undefined *)FUN_ram_00031040(puVar3,puVar5,&DAT_ram_00034358);
LAB_ram_000143d8:
    puVar5 = &DAT_ram_00034370;
    puVar3 = (undefined *)FUN_ram_00031040(puVar3,param_3,&DAT_ram_00034370);
  }
  else {
    if (puVar9 == (undefined *)0x4) {
      puVar3 = param_4 + 5;
      if ((undefined *)0xfffffffffffffffa < param_4) goto LAB_ram_00014360;
      if (puVar3 <= param_3) {
        uVar4 = 3;
        uVar2 = *(uint *)(param_2 + (longlong)puVar7);
        param_2 = (byte *)(ulonglong)(uVar2 >> 0x10);
        param_4 = (undefined *)(ulonglong)(uVar2 >> 8);
        goto LAB_ram_000142c0;
      }
      goto LAB_ram_000143d8;
    }
    if (puVar9 != (undefined *)0x9) goto LAB_ram_00014290;
    puVar3 = param_4 + 9;
    if ((undefined *)0xfffffffffffffff6 < param_4) goto LAB_ram_00014388;
    puVar5 = param_3;
    if (puVar3 <= param_3) {
      if (param_3 <= puVar3) {
                    /* WARNING: Subroutine does not return */
        FUN_ram_0002fc40(puVar3,param_3,&DAT_ram_000343a0);
      }
      uVar2 = (uint)(param_2[(longlong)puVar3] != 0);
      param_2 = param_2 + (longlong)puVar7;
      uVar4 = 4;
      puVar7 = *(undefined **)param_2;
      goto LAB_ram_000142c0;
    }
  }
  param_3 = &DAT_ram_00034388;
  FUN_ram_00031040(puVar3,puVar5,&DAT_ram_00034388);
LAB_ram_00014428:
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fc40(puVar7,param_3,&DAT_ram_00034340);
}

// Function: FUN_ram_000144a0
void FUN_ram_000144a0(undefined1 *param_1,longlong param_2,undefined8 *param_3,longlong param_4)

{
  undefined1 uVar1;
  undefined1 uVar2;
  undefined8 *puVar3;
  undefined *puVar4;
  undefined8 *puVar5;
  undefined *puVar6;
  undefined *puVar7;
  undefined *puVar8;
  ulonglong uVar9;
  longlong lVar10;
  char cStack_28;
  undefined7 uStack_27;
  undefined8 uStack_20;
  longlong lStack_18;
  undefined8 uStack_8;
  
  puVar5 = (undefined8 *)(param_4 + 5);
  if (puVar5 <= param_3) {
    puVar3 = (undefined8 *)(param_4 + 1);
    if (puVar5 < puVar3) {
      puVar7 = &DAT_ram_000343d0;
      FUN_ram_00031048();
      FUN_ram_00013e80(&cStack_28);
      if (cStack_28 != '\a') {
        uStack_8 = uStack_20;
        puVar4 = (undefined *)(lStack_18 + param_4);
        if (puVar4 + 3 <= puVar7) {
          if (puVar7 <= puVar4) {
                    /* WARNING: Subroutine does not return */
            FUN_ram_0002fc40(puVar4,puVar7,&DAT_ram_000343e8);
          }
          puVar6 = puVar4 + 1;
          if (puVar6 < puVar7) {
            puVar8 = puVar4 + 2;
            if (puVar8 < puVar7) {
              uVar1 = *(undefined *)((longlong)puVar5 + (longlong)puVar8);
              uVar2 = *(undefined *)((longlong)puVar5 + (longlong)puVar6);
              *(undefined *)(puVar3 + 2) = *(undefined *)((longlong)puVar5 + (longlong)puVar4);
              *(undefined1 *)((longlong)puVar3 + 0x11) = uVar2;
              *(undefined1 *)((longlong)puVar3 + 0x12) = uVar1;
              puVar3[3] = lStack_18 + 3;
              puVar3[1] = uStack_20;
              *puVar3 = CONCAT71(uStack_27,cStack_28);
              return;
            }
                    /* WARNING: Subroutine does not return */
            FUN_ram_0002fc40(puVar8,puVar7,&DAT_ram_00034418);
          }
                    /* WARNING: Subroutine does not return */
          FUN_ram_0002fc40(puVar6,puVar7,&DAT_ram_00034400);
        }
      }
      *(undefined1 *)puVar3 = 7;
      return;
    }
    if ((ulonglong)*(uint *)(param_2 + (longlong)puVar3) != 0) {
      uVar9 = 0;
      do {
        if (param_3 <= puVar5) goto LAB_ram_00014598;
        lVar10 = 10;
        if ((ulonglong)*(byte *)(param_2 + (longlong)puVar5) < 3) {
          lVar10 = *(longlong *)
                    (&DAT_ram_00033db8 + (ulonglong)*(byte *)(param_2 + (longlong)puVar5) * 8);
        }
        uVar9 = uVar9 + 1;
        puVar5 = (undefined8 *)(lVar10 + (longlong)puVar5);
      } while (uVar9 < *(uint *)(param_2 + (longlong)puVar3));
    }
    if (puVar5 <= param_3) {
      *(longlong *)(param_1 + 0x10) = (longlong)puVar5 - param_4;
      *(longlong *)(param_1 + 8) = ((longlong)puVar5 - param_4) + -1;
      *param_1 = 5;
      return;
    }
  }
LAB_ram_00014598:
  *param_1 = 7;
  return;
}

// Function: FUN_ram_000145f8
void FUN_ram_000145f8(undefined8 *param_1,longlong param_2,ulonglong param_3,longlong param_4)

{
  undefined1 uVar1;
  undefined1 uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  char local_28;
  undefined7 uStack_27;
  undefined8 local_20;
  longlong local_18;
  undefined8 local_8;
  
  FUN_ram_00013e80(&local_28);
  if (local_28 != '\a') {
    local_8 = local_20;
    uVar3 = local_18 + param_4;
    if (uVar3 + 3 <= param_3) {
      if (param_3 <= uVar3) {
                    /* WARNING: Subroutine does not return */
        FUN_ram_0002fc40(uVar3,param_3,&DAT_ram_000343e8);
      }
      uVar4 = uVar3 + 1;
      if (uVar4 < param_3) {
        uVar5 = uVar3 + 2;
        if (uVar5 < param_3) {
          uVar1 = *(undefined1 *)(param_2 + uVar5);
          uVar2 = *(undefined1 *)(param_2 + uVar4);
          *(undefined1 *)(param_1 + 2) = *(undefined1 *)(param_2 + uVar3);
          *(undefined1 *)((longlong)param_1 + 0x11) = uVar2;
          *(undefined1 *)((longlong)param_1 + 0x12) = uVar1;
          param_1[3] = local_18 + 3;
          param_1[1] = local_20;
          *param_1 = CONCAT71(uStack_27,local_28);
          return;
        }
                    /* WARNING: Subroutine does not return */
        FUN_ram_0002fc40(uVar5,param_3,&DAT_ram_00034418);
      }
                    /* WARNING: Subroutine does not return */
      FUN_ram_0002fc40(uVar4,param_3,&DAT_ram_00034400);
    }
  }
  *(undefined1 *)param_1 = 7;
  return;
}

// Function: FUN_ram_000147d8
void FUN_ram_000147d8(undefined8 *param_1,longlong param_2,ulonglong param_3,longlong param_4)

{
  undefined1 uVar1;
  undefined1 uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  char local_28;
  undefined7 uStack_27;
  undefined8 local_20;
  longlong local_18;
  undefined8 local_8;
  
  FUN_ram_00013e80(&local_28);
  if (local_28 == '\a') {
LAB_ram_00014870:
    *(undefined1 *)param_1 = 7;
    return;
  }
  local_8 = local_20;
  uVar3 = local_18 + param_4;
  if (param_3 < uVar3 + 4) goto LAB_ram_00014870;
  uVar4 = uVar3 + 2;
  if (uVar3 < 0xfffffffffffffffe) {
    if (uVar4 <= param_3) {
      uVar5 = uVar4;
      if (uVar4 < param_3) {
        uVar5 = uVar3 + 3;
        if (uVar5 < param_3) {
          uVar1 = *(undefined1 *)(param_2 + uVar5);
          uVar2 = *(undefined1 *)(param_2 + uVar4);
          *(undefined2 *)(param_1 + 2) = *(undefined2 *)(param_2 + uVar3);
          *(undefined1 *)((longlong)param_1 + 0x12) = uVar2;
          *(undefined1 *)((longlong)param_1 + 0x13) = uVar1;
          param_1[3] = local_18 + 4;
          param_1[1] = local_20;
          *param_1 = CONCAT71(uStack_27,local_28);
          return;
        }
                    /* WARNING: Subroutine does not return */
        FUN_ram_0002fc40(uVar5,param_3,&DAT_ram_00034460);
      }
      goto LAB_ram_00014990;
    }
  }
  else {
    FUN_ram_00031048(uVar3,uVar4,&DAT_ram_00034430);
  }
  uVar5 = param_3;
  FUN_ram_00031040(uVar4,param_3,&DAT_ram_00034430);
LAB_ram_00014990:
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fc40(uVar5,param_3,&DAT_ram_00034448);
}

// Function: FUN_ram_000149e0
/* WARNING: Type propagation algorithm not settling */

void FUN_ram_000149e0(undefined4 *param_1,longlong *param_2,undefined4 *param_3)

{
  uint uVar1;
  undefined2 uVar2;
  undefined2 uVar3;
  undefined2 uVar4;
  char cVar5;
  longlong lVar6;
  undefined8 uVar7;
  undefined1 uVar8;
  undefined8 *puVar9;
  undefined4 *puVar10;
  undefined4 *puVar11;
  undefined8 uVar12;
  undefined1 uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  undefined4 *unaff_R6;
  uint uVar16;
  ulonglong uVar17;
  undefined4 *puVar18;
  byte local_398;
  undefined1 local_380;
  undefined4 *local_340;
  undefined8 local_338;
  undefined1 local_330 [4];
  undefined1 auStack_32c [5];
  undefined8 local_327;
  undefined1 auStack_31f [7];
  longlong local_318 [46];
  undefined1 local_1a8 [4];
  undefined1 auStack_1a4 [4];
  char cStack_1a0;
  undefined8 local_19f;
  undefined1 auStack_197 [7];
  longlong local_190 [47];
  undefined8 local_18;
  
  if (param_3 < (undefined4 *)0x8) goto LAB_ram_00014ed0;
  lVar6 = *param_2;
  if (lVar6 < 0x24f3f41552b88a9d) {
    if (lVar6 < -0x162701836cac672f) {
      uVar13 = 2;
      if (lVar6 == -0x7e6329becc64df3f) {
LAB_ram_00014e98:
        puVar11 = (undefined4 *)0x1;
        puVar18 = (undefined4 *)0x0;
        lVar6 = 0x13;
        if (param_3 < (undefined4 *)0x20) goto LAB_ram_00014ed0;
      }
      else {
        if (lVar6 != -0x55956088af70861a) goto LAB_ram_00014ed0;
        puVar18 = (undefined4 *)0x1;
        lVar6 = 0xb;
        uVar13 = 3;
        puVar11 = (undefined4 *)0x1;
        if (param_3 < (undefined4 *)0x18) goto LAB_ram_00014ed0;
      }
      goto LAB_ram_00014f20;
    }
    if (lVar6 == -0x162701836cac672f) {
      uVar13 = 7;
    }
    else {
      uVar13 = 6;
      if (lVar6 == 0x14afc431ccfa64bb) goto LAB_ram_00014c18;
      if (lVar6 != 0x18fabbd8cae56035) goto LAB_ram_00014ed0;
      uVar13 = 9;
    }
    if (param_3 < (undefined4 *)0x23) goto LAB_ram_00014ed0;
    lVar6 = 9;
    uVar8 = 1;
    local_398 = *(byte *)(param_2 + 1);
    unaff_R6 = (undefined4 *)(ulonglong)local_398;
LAB_ram_00014c68:
    puVar18 = (undefined4 *)(lVar6 + 0x1a);
    if (puVar18 <= param_3) {
      puVar9 = (undefined8 *)((longlong)param_2 + lVar6);
      uVar2 = *(undefined2 *)((longlong)puVar9 + 0x14);
      uVar3 = *(undefined2 *)((longlong)puVar9 + 0x12);
      uVar4 = *(undefined2 *)(puVar9 + 2);
      uVar12 = puVar9[1];
      uVar7 = *puVar9;
      uVar1 = *(uint *)((longlong)puVar9 + 0x16);
      if ((ulonglong)uVar1 == 0) {
        uVar16 = 0;
      }
      else {
        uVar17 = 0;
        unaff_R6 = (undefined4 *)((longlong)local_1a8 + 1);
        uVar16 = 0;
        do {
          FUN_ram_000147d8(local_1a8,param_2,param_3,puVar18);
          if (local_1a8[0] == '\a') goto LAB_ram_00015160;
          local_18 = CONCAT53(_auStack_1a4,local_1a8._1_3_);
          if (uVar17 < 0x10) {
            uVar14 = (ulonglong)uVar16;
            if (0xf < uVar14) goto LAB_ram_000156a8;
            auStack_32c[uVar14 * 0x18 + 4] = local_1a8[0];
            (&local_327)[uVar14 * 3] = local_18;
            *(ulonglong *)((longlong)local_318 + uVar14 * 0x18 + -7) =
                 CONCAT17(local_19f._7_1_,(undefined7)local_19f);
            local_318[uVar14 * 3] = CONCAT71(auStack_197,local_19f._7_1_);
            uVar16 = uVar16 + 1;
          }
          uVar17 = uVar17 + 1;
          puVar18 = (undefined4 *)((longlong)puVar18 + local_190[0]);
        } while (uVar17 < uVar1);
      }
      local_330 = (undefined1  [4])uVar16;
      FUN_ram_00031b28(auStack_1a4 + 2,local_330,0x188);
      *(byte *)((longlong)param_1 + 9) = local_398;
      *(undefined1 *)(param_1 + 2) = uVar8;
      *(undefined2 *)((longlong)param_1 + 6) = uVar2;
      *(undefined2 *)(param_1 + 1) = uVar3;
      *(undefined2 *)((longlong)param_1 + 2) = uVar4;
      *(undefined1 *)((longlong)param_1 + 1) = uVar13;
      *(undefined1 *)param_1 = 1;
      FUN_ram_00031b28((undefined1 *)((longlong)param_1 + 10),local_1a8,0x18e);
      *(undefined8 *)(param_1 + 0x68) = uVar12;
      *(undefined8 *)(param_1 + 0x66) = uVar7;
      return;
    }
    FUN_ram_00031040(puVar18,param_3,&DAT_ram_00034538);
    puVar10 = param_3;
    puVar18 = param_1;
LAB_ram_000154b0:
    FUN_ram_00031048();
LAB_ram_000154c8:
    param_3 = (undefined4 *)&DAT_ram_00034478;
    FUN_ram_00031048();
LAB_ram_000154e0:
    puVar11 = (undefined4 *)&DAT_ram_000344d8;
    FUN_ram_00031040(puVar10,param_3,&DAT_ram_000344d8);
    puVar10 = param_3;
    param_3 = puVar11;
LAB_ram_00015508:
    FUN_ram_00031040(puVar10,param_3,&DAT_ram_00034478);
    puVar10 = param_3;
LAB_ram_00015530:
    puVar11 = unaff_R6;
    FUN_ram_00031048(puVar10,unaff_R6,&DAT_ram_000344f0);
    puVar10 = puVar11;
LAB_ram_00015558:
    param_3 = (undefined4 *)&DAT_ram_00034490;
    FUN_ram_00031048(puVar10,puVar18,&DAT_ram_00034490);
LAB_ram_00015580:
    puVar11 = (undefined4 *)&DAT_ram_000344f0;
    FUN_ram_00031040(unaff_R6,param_3,&DAT_ram_000344f0);
    param_3 = puVar11;
LAB_ram_000155a8:
    FUN_ram_00031040(puVar18,param_3,&DAT_ram_00034490);
LAB_ram_000155d0:
    param_3 = (undefined4 *)&DAT_ram_000344a8;
    FUN_ram_00031048(puVar18,unaff_R6);
  }
  else {
    if (lVar6 < 0x3e457d9aa869d1b0) {
      if (lVar6 == 0x24f3f41552b88a9d) {
        uVar13 = 8;
LAB_ram_00014c18:
        if (param_3 < (undefined4 *)0x22) goto LAB_ram_00014ed0;
        lVar6 = 8;
        uVar8 = 0;
        goto LAB_ram_00014c68;
      }
      uVar13 = 0;
      if (lVar6 != 0x2aade37a97cb17e5) goto LAB_ram_00014ed0;
    }
    else {
      if (lVar6 == 0x3e457d9aa869d1b0) {
        uVar13 = 5;
        goto LAB_ram_00014e98;
      }
      if (lVar6 == 0x680e5da774475696) {
        puVar11 = (undefined4 *)0x0;
        lVar6 = 0xb;
        uVar13 = 1;
        puVar18 = (undefined4 *)0x1;
        if (param_3 < (undefined4 *)0x17) goto LAB_ram_00014ed0;
        goto LAB_ram_00014f20;
      }
      if (lVar6 != 0x5ced2b7b97ef33d0) goto LAB_ram_00014ed0;
      uVar13 = 4;
    }
    puVar18 = (undefined4 *)0x0;
    lVar6 = 0x13;
    puVar11 = (undefined4 *)0x0;
    if (param_3 < (undefined4 *)0x1f) {
LAB_ram_00014ed0:
      *(undefined1 *)param_1 = 2;
      return;
    }
LAB_ram_00014f20:
    if (puVar11 == (undefined4 *)0x0) {
      uVar17 = 8;
    }
    else {
      uVar17 = 9;
      local_380 = (undefined1)param_2[1];
    }
    uVar14 = uVar17 | 4;
    uVar1 = *(uint *)((longlong)param_2 + uVar17);
    if ((ulonglong)uVar1 == 0) {
      uVar16 = 0;
      unaff_R6 = puVar11;
    }
    else {
      uVar17 = 0;
      unaff_R6 = (undefined4 *)((longlong)local_330 + 1);
      uVar16 = 0;
      do {
        FUN_ram_000145f8(local_330,param_2,param_3,uVar14);
        if (local_330[0] == '\a') goto LAB_ram_00015160;
        local_18 = CONCAT53(auStack_32c,local_330._1_3_);
        if (uVar17 < 0x10) {
          uVar15 = (ulonglong)uVar16;
          if (0xf < uVar15) goto LAB_ram_00015620;
          (&cStack_1a0)[uVar15 * 0x18] = local_330[0];
          (&local_19f)[uVar15 * 3] = local_18;
          *(ulonglong *)((longlong)local_190 + uVar15 * 0x18 + -7) =
               CONCAT17(local_327._7_1_,(undefined7)local_327);
          local_190[uVar15 * 3] = CONCAT71(auStack_31f,local_327._7_1_);
          uVar16 = uVar16 + 1;
        }
        uVar17 = uVar17 + 1;
        uVar14 = local_318[0] + uVar14;
      } while (uVar17 < uVar1);
    }
    if (param_3 < (undefined4 *)(uVar14 + lVar6)) {
LAB_ram_00015160:
      *(undefined1 *)param_1 = 2;
      return;
    }
    puVar10 = (undefined4 *)(uVar14 + 8);
    local_1a8 = (undefined1  [4])uVar16;
    if (puVar18 != (undefined4 *)0x0) {
      if (0xfffffffffffffff7 < uVar14) goto LAB_ram_000154b0;
      if (param_3 < puVar10) goto LAB_ram_000154e0;
      unaff_R6 = (undefined4 *)(uVar14 + 10);
      if (unaff_R6 < puVar10) goto LAB_ram_00015530;
      if (unaff_R6 <= param_3) {
        if (unaff_R6 < param_3) {
          local_338 = *(undefined8 *)((longlong)param_2 + uVar14);
          uVar7 = 0;
          puVar18 = puVar10;
          local_340 = param_3;
LAB_ram_000153b0:
          uVar8 = *(undefined1 *)((longlong)param_2 + (longlong)unaff_R6);
          uVar2 = *(undefined2 *)((longlong)param_2 + (longlong)puVar18);
          FUN_ram_00031b28(param_1 + 2,local_1a8,0x188);
          *(undefined4 **)(param_1 + 0x68) = local_340;
          *(undefined8 *)(param_1 + 0x66) = uVar7;
          *(undefined8 *)(param_1 + 100) = local_338;
          *(undefined1 *)((longlong)param_1 + 7) = local_380;
          *(char *)((longlong)param_1 + 6) = (char)puVar11;
          *(undefined2 *)(param_1 + 1) = uVar2;
          *(undefined1 *)((longlong)param_1 + 2) = uVar13;
          *(undefined1 *)((longlong)param_1 + 1) = uVar8;
          *(undefined1 *)param_1 = 0;
          return;
        }
        goto LAB_ram_00015730;
      }
      goto LAB_ram_00015580;
    }
    if (0xfffffffffffffff7 < uVar14) goto LAB_ram_000154c8;
    if (param_3 < puVar10) goto LAB_ram_00015508;
    puVar18 = (undefined4 *)(uVar14 + 0x10);
    if (puVar18 < puVar10) goto LAB_ram_00015558;
    if (param_3 < puVar18) goto LAB_ram_000155a8;
    unaff_R6 = (undefined4 *)(uVar14 + 0x12);
    if (unaff_R6 < puVar18) goto LAB_ram_000155d0;
    if (unaff_R6 <= param_3) {
      if (param_3 <= unaff_R6) {
                    /* WARNING: Subroutine does not return */
        FUN_ram_0002fc40(unaff_R6,param_3,&DAT_ram_000344c0);
      }
      local_340 = *(undefined4 **)((longlong)param_2 + uVar14);
      local_338 = *(undefined8 *)((longlong)param_2 + (longlong)puVar10);
      uVar7 = 1;
      goto LAB_ram_000153b0;
    }
  }
  FUN_ram_00031040(unaff_R6,param_3,&DAT_ram_000344a8);
  local_330[0] = (char)param_3;
LAB_ram_00015620:
  local_330._1_3_ = (undefined3)local_18;
  auStack_32c = SUB85((ulonglong)local_18 >> 0x18,0);
  cVar5 = '+';
  FUN_ram_0002fd08(&DAT_ram_00033764,0x2b,local_330,&DAT_ram_00034130,&DAT_ram_00034520);
  local_1a8[0] = cVar5;
LAB_ram_000156a8:
  local_1a8._1_3_ = (undefined3)local_18;
  _auStack_1a4 = (undefined5)((ulonglong)local_18 >> 0x18);
  param_3 = (undefined4 *)local_1a8;
  FUN_ram_0002fd08(&DAT_ram_00033764,0x2b,param_3,&DAT_ram_00034190,&DAT_ram_00034550);
LAB_ram_00015730:
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fc40(unaff_R6,param_3,&DAT_ram_00034508);
}

// Function: FUN_ram_00015780
bool FUN_ram_00015780(char *param_1,char param_2)

{
  byte bVar1;
  char *pcVar2;
  char *pcVar3;
  longlong lVar4;
  char cVar5;
  
  if (*param_1 == '\0') {
    lVar4 = (ulonglong)*(uint *)(param_1 + 8) * 0x18;
    pcVar2 = param_1 + -6;
    do {
      pcVar3 = pcVar2;
      if (lVar4 == 0) {
        return (bool)2;
      }
      cVar5 = 'o';
      bVar1 = pcVar3[0x16];
      if (bVar1 != 5) {
        cVar5 = pcVar3[0x17];
      }
      lVar4 = lVar4 + -0x18;
      pcVar2 = pcVar3 + 0x18;
    } while (cVar5 != param_2);
    if (bVar1 < 3) {
      if (bVar1 == 0) {
        return (bool)2;
      }
      if (bVar1 == 1) {
LAB_ram_000158d8:
        return (bool)*pcVar2;
      }
    }
    else {
      if (4 < bVar1) {
        if (bVar1 == 5) {
          return (bool)2;
        }
        if (*pcVar2 != '\x01') {
          return (bool)2;
        }
        cVar5 = pcVar3[0x19];
        goto code_r0x00015978;
      }
      if (bVar1 != 3) goto LAB_ram_000158d8;
    }
    cVar5 = *pcVar2;
  }
  else {
    lVar4 = (ulonglong)*(uint *)(param_1 + 0x10) * 0x18;
    do {
      pcVar2 = param_1;
      if (lVar4 == 0) {
        return (bool)2;
      }
      cVar5 = 'o';
      bVar1 = pcVar2[0x18];
      if (bVar1 != 5) {
        cVar5 = pcVar2[0x19];
      }
      lVar4 = lVar4 + -0x18;
      param_1 = pcVar2 + 0x18;
    } while (cVar5 != param_2);
    if (bVar1 < 3) {
      if (bVar1 == 0) {
        return (bool)2;
      }
      if (bVar1 == 1) {
LAB_ram_000158b0:
        return (bool)pcVar2[0x1a];
      }
    }
    else {
      if (4 < bVar1) {
        if (bVar1 == 5) {
          return (bool)2;
        }
        if (pcVar2[0x1a] != '\x01') {
          return (bool)2;
        }
        cVar5 = pcVar2[0x1b];
        goto code_r0x00015978;
      }
      if (bVar1 != 3) goto LAB_ram_000158b0;
    }
    cVar5 = pcVar2[0x1a];
  }
code_r0x00015978:
  return cVar5 != '\0';
}

// Function: FUN_ram_000159c0
undefined8 FUN_ram_000159c0(char *param_1)

{
  int iVar1;
  char cVar2;
  char cVar3;
  bool bVar4;
  char *pcVar5;
  undefined8 uVar6;
  longlong *plVar7;
  longlong lVar8;
  char *pcVar9;
  longlong *plVar10;
  longlong lVar11;
  undefined4 *puVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  longlong local_70 [2];
  uint local_5c;
  char local_58 [4];
  char local_54;
  char local_53;
  char local_52;
  char local_51;
  char local_50;
  char local_4f;
  char local_4e;
  char local_4d;
  char local_4c;
  char local_4b;
  char local_4a;
  char local_49;
  char local_48;
  char local_47;
  char local_46;
  char local_45;
  char local_44;
  char local_43;
  char local_42;
  char local_41;
  char local_40;
  char local_3f;
  char local_3e;
  char local_3d;
  char local_3c;
  char local_3b;
  char local_3a;
  char local_39;
  undefined4 local_38;
  char local_34 [32];
  uint local_14;
  char local_10 [16];
  
  if (*param_1 != '\0') {
    iVar1 = *(int *)(param_1 + 0x10);
    if (iVar1 == 0) {
      return 0;
    }
    local_58[1] = param_1[0x2b];
    local_58[0] = param_1[0x2a];
    lVar11 = 1;
    if (iVar1 == 1) goto LAB_ram_00016070;
    local_58[3] = param_1[0x43];
    local_58[2] = param_1[0x42];
    lVar11 = 2;
    if (iVar1 == 2) goto LAB_ram_00016070;
    local_53 = param_1[0x5b];
    local_54 = param_1[0x5a];
    lVar11 = 3;
    if (iVar1 == 3) goto LAB_ram_00016070;
    local_51 = param_1[0x73];
    local_52 = param_1[0x72];
    lVar11 = 4;
    if (iVar1 == 4) goto LAB_ram_00016070;
    local_4f = param_1[0x8b];
    local_50 = param_1[0x8a];
    lVar11 = 5;
    if (iVar1 == 5) goto LAB_ram_00016070;
    local_4d = param_1[0xa3];
    local_4e = param_1[0xa2];
    lVar11 = 6;
    if (iVar1 == 6) goto LAB_ram_00016070;
    local_4b = param_1[0xbb];
    local_4c = param_1[0xba];
    lVar11 = 7;
    if (iVar1 == 7) goto LAB_ram_00016070;
    local_49 = param_1[0xd3];
    local_4a = param_1[0xd2];
    lVar11 = 8;
    if (iVar1 == 8) goto LAB_ram_00016070;
    local_47 = param_1[0xeb];
    local_48 = param_1[0xea];
    lVar11 = 9;
    if (iVar1 == 9) goto LAB_ram_00016070;
    local_45 = param_1[0x103];
    local_46 = param_1[0x102];
    lVar11 = 10;
    if (iVar1 == 10) goto LAB_ram_00016070;
    local_43 = param_1[0x11b];
    local_44 = param_1[0x11a];
    lVar11 = 0xb;
    if (iVar1 == 0xb) goto LAB_ram_00016070;
    local_41 = param_1[0x133];
    local_42 = param_1[0x132];
    lVar11 = 0xc;
    if (iVar1 == 0xc) goto LAB_ram_00016070;
    local_3f = param_1[0x14b];
    local_40 = param_1[0x14a];
    lVar11 = 0xd;
    if (iVar1 == 0xd) goto LAB_ram_00016070;
    local_3d = param_1[0x163];
    local_3e = param_1[0x162];
    lVar11 = 0xe;
    if (iVar1 == 0xe) goto LAB_ram_00016070;
    local_3b = param_1[0x17b];
    local_3c = param_1[0x17a];
    lVar11 = 0xf;
    if (iVar1 == 0xf) goto LAB_ram_00016070;
    local_39 = param_1[0x193];
    local_3a = param_1[0x192];
    lVar11 = 0x10;
    if (iVar1 == 0x10) goto LAB_ram_00016070;
    local_38 = CONCAT31(CONCAT21(local_38._2_2_,param_1[0x1ab]),param_1[0x1aa]);
    param_1 = &DAT_ram_00033764;
    FUN_ram_0002fd08(&DAT_ram_00033764,0x2b,&local_38,&DAT_ram_00034150,&DAT_ram_00034580);
  }
  if ((ulonglong)*(uint *)(param_1 + 8) == 0) {
    return 0;
  }
  lVar8 = (ulonglong)*(uint *)(param_1 + 8) * 0x18;
  local_58[1] = param_1[0x22];
  local_58[0] = param_1[0x21];
  lVar11 = 1;
  if (lVar8 != 0x18) {
    local_58[3] = param_1[0x3a];
    local_58[2] = param_1[0x39];
    lVar11 = 2;
    if (lVar8 != 0x30) {
      local_53 = param_1[0x52];
      local_54 = param_1[0x51];
      lVar11 = 3;
      if (lVar8 != 0x48) {
        local_51 = param_1[0x6a];
        local_52 = param_1[0x69];
        lVar11 = 4;
        if (lVar8 != 0x60) {
          local_4f = param_1[0x82];
          local_50 = param_1[0x81];
          lVar11 = 5;
          if (lVar8 != 0x78) {
            local_4d = param_1[0x9a];
            local_4e = param_1[0x99];
            lVar11 = 6;
            if (lVar8 != 0x90) {
              local_4b = param_1[0xb2];
              local_4c = param_1[0xb1];
              lVar11 = 7;
              if (lVar8 != 0xa8) {
                local_49 = param_1[0xca];
                local_4a = param_1[0xc9];
                lVar11 = 8;
                if (lVar8 != 0xc0) {
                  local_47 = param_1[0xe2];
                  local_48 = param_1[0xe1];
                  lVar11 = 9;
                  if (lVar8 != 0xd8) {
                    local_45 = param_1[0xfa];
                    local_46 = param_1[0xf9];
                    lVar11 = 10;
                    if (lVar8 != 0xf0) {
                      local_43 = param_1[0x112];
                      local_44 = param_1[0x111];
                      lVar11 = 0xb;
                      if (lVar8 != 0x108) {
                        local_41 = param_1[0x12a];
                        local_42 = param_1[0x129];
                        lVar11 = 0xc;
                        if (lVar8 != 0x120) {
                          local_3f = param_1[0x142];
                          local_40 = param_1[0x141];
                          lVar11 = 0xd;
                          if (lVar8 != 0x138) {
                            local_3d = param_1[0x15a];
                            local_3e = param_1[0x159];
                            lVar11 = 0xe;
                            if (lVar8 != 0x150) {
                              local_3b = param_1[0x172];
                              local_3c = param_1[0x171];
                              lVar11 = 0xf;
                              if (lVar8 != 0x168) {
                                local_39 = param_1[0x18a];
                                local_3a = param_1[0x189];
                                lVar11 = 0x10;
                                if (lVar8 != 0x180) {
                                  local_38 = CONCAT31(CONCAT21(local_38._2_2_,param_1[0x1a2]),
                                                      param_1[0x1a1]);
                                  puVar12 = &local_38;
                                  plVar7 = (longlong *)&DAT_ram_00033764;
                                  plVar10 = (longlong *)0x2b;
                                  FUN_ram_0002fd08();
                                  uVar6 = 0xd;
                                  if ((undefined4 *)0x7 < puVar12) {
                                    if ((((*plVar7 != 0x6ec031f25bd57904) ||
                                         (plVar7[1] != 0x71568ce6ec574ee)) ||
                                        (plVar7[2] != 0x518ef4a3deb2b1fd)) ||
                                       (bVar4 = false, plVar7[3] != -0x70ec43a95d324efe)) {
                                      bVar4 = true;
                                    }
                                    lVar11 = *plVar10;
                                    if (bVar4) {
                                      if (((*plVar7 != 0x4873bce2144ae3b5) ||
                                          (plVar7[1] != -0x2911a2500a1ef197)) ||
                                         ((plVar7[2] != 0x60b8aa6da3403855 ||
                                          (bVar4 = false, plVar7[3] != 0x103cc0bd736050b0)))) {
                                        bVar4 = true;
                                      }
                                      if (bVar4) {
                                        if ((((*plVar7 != -0x44f118ed916356fa) ||
                                             (plVar7[1] != 0x6e904b4c145c1835)) ||
                                            (plVar7[2] != 0x2a2f74470ab0ff18)) ||
                                           (bVar4 = false, plVar7[3] != -0x2b367796f4eefba2)) {
                                          bVar4 = true;
                                        }
                                        if (bVar4) {
                                          if (((*plVar7 != 0x136d5ca2f1569155) ||
                                              (plVar7[1] != 0x340d9a0ae6f72a4f)) ||
                                             ((plVar7[2] != -0x2a9d9b9ca96e3882 ||
                                              (bVar4 = false, plVar7[3] != 0x698f3435f126add1)))) {
                                            bVar4 = true;
                                          }
                                          if ((!bVar4) &&
                                             (((lVar11 == 0x19f106ccead8aadf ||
                                               (lVar11 == 0x3cec9b1033d4c9bb)) ||
                                              (lVar11 == 0x351f5084b15529aa)))) {
                                            uVar6 = 0xc;
                                          }
                                        }
                                        else if (lVar11 == -0x78ffe9badeaba407) {
                                          uVar6 = 0xb;
                                        }
                                      }
                                      else if (((lVar11 == -0x77a4a414b3c0b4bf) ||
                                               (lVar11 == 0x65879cc54d18aca8)) ||
                                              (lVar11 == -0x37788a1e6e613908)) {
                                        uVar6 = 10;
                                      }
                                    }
                                    else {
                                      if (lVar11 < 0x24f3f41552b88a9d) {
                                        if (lVar11 < -0x162701836cac672f) {
                                          if (lVar11 == -0x7e6329becc64df3f) {
                                            return 2;
                                          }
                                          if (lVar11 == -0x55956088af70861a) {
                                            return 3;
                                          }
                                        }
                                        else {
                                          if (lVar11 == -0x162701836cac672f) {
                                            return 7;
                                          }
                                          if (lVar11 == 0x14afc431ccfa64bb) {
                                            return 6;
                                          }
                                          if (lVar11 == 0x18fabbd8cae56035) {
                                            return 9;
                                          }
                                        }
                                      }
                                      else if (lVar11 < 0x3e457d9aa869d1b0) {
                                        if (lVar11 == 0x24f3f41552b88a9d) {
                                          return 8;
                                        }
                                        if (lVar11 == 0x2aade37a97cb17e5) {
                                          return 0;
                                        }
                                      }
                                      else {
                                        if (lVar11 == 0x3e457d9aa869d1b0) {
                                          return 5;
                                        }
                                        if (lVar11 == 0x5ced2b7b97ef33d0) {
                                          return 4;
                                        }
                                        if (lVar11 == 0x680e5da774475696) {
                                          return 1;
                                        }
                                      }
                                      uVar6 = 0xd;
                                    }
                                  }
                                  return uVar6;
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
LAB_ram_00016070:
  local_5c = (uint)lVar11;
  pcVar5 = local_58;
  do {
    cVar2 = *pcVar5;
    if ((ulonglong)local_5c == 0) {
      uVar14 = 0;
    }
    else {
      uVar14 = 0;
      pcVar9 = local_58;
      do {
        if (*pcVar9 == cVar2) {
          if (pcVar9[1] == cVar2) {
            return 1;
          }
          if ((uVar14 & 0xffffffff) < 0x20) {
            local_34[uVar14 & 0xffffffff] = pcVar9[1];
            uVar14 = uVar14 + 1;
          }
        }
        pcVar9 = pcVar9 + 2;
      } while (pcVar9 != local_58 + (ulonglong)local_5c * 2);
    }
    local_14 = 0;
LAB_ram_00016228:
    if ((uVar14 & 0xffffffff) != 0) {
      uVar13 = (ulonglong)local_14;
      uVar15 = uVar14 & 0xffffffff;
      uVar14 = uVar15 - 1;
      local_38 = (int)uVar14;
      cVar3 = local_34[uVar15 - 1];
      if (cVar3 == cVar2) {
        return 1;
      }
      if (0xf < uVar13) break;
      if (uVar13 != 0) {
        uVar15 = 0;
        do {
          if (local_10[uVar15] == cVar3) goto LAB_ram_00016228;
          uVar15 = uVar15 + 1;
        } while (uVar15 < uVar13);
        goto joined_r0x00016340;
      }
      uVar13 = 0;
      goto LAB_ram_00016368;
    }
    pcVar5 = pcVar5 + 2;
    if (pcVar5 == local_58 + lVar11 * 2) {
      return 0;
    }
  } while( true );
  FUN_ram_00030e00(local_70,cVar3,local_10);
  if (local_70[0] != 1) {
    uVar13 = (ulonglong)local_14;
joined_r0x00016340:
    if (uVar13 < 0x10) {
LAB_ram_00016368:
      local_10[uVar13] = cVar3;
      local_14 = (int)uVar13 + 1;
    }
    if ((ulonglong)local_5c != 0) {
      lVar8 = (ulonglong)local_5c << 1;
      pcVar9 = local_58 + 1;
      do {
        if ((pcVar9[-1] == cVar3) && ((uVar14 & 0xffffffff) < 0x20)) {
          local_34[uVar14 & 0xffffffff] = *pcVar9;
          uVar14 = uVar14 + 1;
        }
        pcVar9 = pcVar9 + 2;
        lVar8 = lVar8 + -2;
      } while (lVar8 != 0);
    }
    local_38 = (int)uVar14;
  }
  goto LAB_ram_00016228;
}

// Function: FUN_ram_000164d8
undefined8 FUN_ram_000164d8(longlong *param_1,longlong *param_2,ulonglong param_3)

{
  bool bVar1;
  undefined8 uVar2;
  longlong lVar3;
  
  uVar2 = 0xd;
  if (7 < param_3) {
    if ((((*param_1 != 0x6ec031f25bd57904) || (param_1[1] != 0x71568ce6ec574ee)) ||
        (param_1[2] != 0x518ef4a3deb2b1fd)) || (bVar1 = false, param_1[3] != -0x70ec43a95d324efe)) {
      bVar1 = true;
    }
    lVar3 = *param_2;
    if (bVar1) {
      if (((*param_1 != 0x4873bce2144ae3b5) || (param_1[1] != -0x2911a2500a1ef197)) ||
         ((param_1[2] != 0x60b8aa6da3403855 || (bVar1 = false, param_1[3] != 0x103cc0bd736050b0))))
      {
        bVar1 = true;
      }
      if (bVar1) {
        if ((((*param_1 != -0x44f118ed916356fa) || (param_1[1] != 0x6e904b4c145c1835)) ||
            (param_1[2] != 0x2a2f74470ab0ff18)) ||
           (bVar1 = false, param_1[3] != -0x2b367796f4eefba2)) {
          bVar1 = true;
        }
        if (bVar1) {
          if (((*param_1 != 0x136d5ca2f1569155) || (param_1[1] != 0x340d9a0ae6f72a4f)) ||
             ((param_1[2] != -0x2a9d9b9ca96e3882 ||
              (bVar1 = false, param_1[3] != 0x698f3435f126add1)))) {
            bVar1 = true;
          }
          if ((!bVar1) &&
             (((lVar3 == 0x19f106ccead8aadf || (lVar3 == 0x3cec9b1033d4c9bb)) ||
              (lVar3 == 0x351f5084b15529aa)))) {
            uVar2 = 0xc;
          }
        }
        else if (lVar3 == -0x78ffe9badeaba407) {
          uVar2 = 0xb;
        }
      }
      else if (((lVar3 == -0x77a4a414b3c0b4bf) || (lVar3 == 0x65879cc54d18aca8)) ||
              (lVar3 == -0x37788a1e6e613908)) {
        uVar2 = 10;
      }
    }
    else {
      if (lVar3 < 0x24f3f41552b88a9d) {
        if (lVar3 < -0x162701836cac672f) {
          if (lVar3 == -0x7e6329becc64df3f) {
            return 2;
          }
          if (lVar3 == -0x55956088af70861a) {
            return 3;
          }
        }
        else {
          if (lVar3 == -0x162701836cac672f) {
            return 7;
          }
          if (lVar3 == 0x14afc431ccfa64bb) {
            return 6;
          }
          if (lVar3 == 0x18fabbd8cae56035) {
            return 9;
          }
        }
      }
      else if (lVar3 < 0x3e457d9aa869d1b0) {
        if (lVar3 == 0x24f3f41552b88a9d) {
          return 8;
        }
        if (lVar3 == 0x2aade37a97cb17e5) {
          return 0;
        }
      }
      else {
        if (lVar3 == 0x3e457d9aa869d1b0) {
          return 5;
        }
        if (lVar3 == 0x5ced2b7b97ef33d0) {
          return 4;
        }
        if (lVar3 == 0x680e5da774475696) {
          return 1;
        }
      }
      uVar2 = 0xd;
    }
  }
  return uVar2;
}

// Function: FUN_ram_00016a10
void FUN_ram_00016a10(undefined4 *param_1,uint *param_2)

{
  uint uVar1;
  uint uVar2;
  uint uVar3;
  undefined4 uVar4;
  
  uVar4 = 0;
  uVar1 = *param_2;
  if (50000 < uVar1) goto LAB_ram_00016ac8;
  uVar2 = param_2[2];
  if (uVar2 != uVar1) {
    if (((50000 < uVar2) || (uVar2 <= uVar1)) || (param_2[3] < param_2[1])) goto LAB_ram_00016ac8;
    uVar3 = param_2[4];
    uVar1 = uVar2;
    if (uVar3 != uVar2) {
      if (((50000 < uVar3) || (uVar3 <= uVar2)) || (param_2[5] < param_2[3])) goto LAB_ram_00016ac8;
      uVar2 = param_2[6];
      uVar1 = uVar3;
      if (uVar2 != uVar3) {
        if (((50000 < uVar2) || (uVar2 <= uVar3)) || (param_2[7] < param_2[5]))
        goto LAB_ram_00016ac8;
        uVar3 = param_2[8];
        uVar1 = uVar2;
        if (uVar3 != uVar2) {
          if (((50000 < uVar3) || (uVar3 <= uVar2)) || (param_2[9] < param_2[7]))
          goto LAB_ram_00016ac8;
          uVar2 = param_2[10];
          uVar1 = uVar3;
          if (uVar2 != uVar3) {
            if (((50000 < uVar2) || (uVar2 <= uVar3)) || (param_2[0xb] < param_2[9]))
            goto LAB_ram_00016ac8;
            uVar3 = param_2[0xc];
            uVar1 = uVar2;
            if (uVar3 != uVar2) {
              if (((50000 < uVar3) || (uVar3 <= uVar2)) || (param_2[0xd] < param_2[0xb]))
              goto LAB_ram_00016ac8;
              uVar2 = param_2[0xe];
              uVar1 = uVar3;
              if (uVar2 != uVar3) {
                if (((50000 < uVar2) || (uVar2 <= uVar3)) || (param_2[0xf] < param_2[0xd]))
                goto LAB_ram_00016ac8;
                uVar3 = param_2[0x10];
                uVar1 = uVar2;
                if (uVar3 != uVar2) {
                  if (((50000 < uVar3) || (uVar3 <= uVar2)) || (param_2[0x11] < param_2[0xf]))
                  goto LAB_ram_00016ac8;
                  uVar2 = param_2[0x12];
                  uVar1 = uVar3;
                  if (uVar2 != uVar3) {
                    if (((uVar2 < 0xc351) && (uVar3 < uVar2)) && (param_2[0x11] <= param_2[0x13])) {
                      uVar4 = 0x1a;
                    }
                    goto LAB_ram_00016ac8;
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  uVar4 = 0x1a;
  if (uVar1 != 50000) {
    uVar4 = 0;
  }
LAB_ram_00016ac8:
  *param_1 = uVar4;
  param_1[1] = 0xbad3;
  return;
}

// Function: FUN_ram_00016c40
ulonglong FUN_ram_00016c40(ulonglong param_1,longlong param_2)

{
  if (param_2 != 0x6c0) {
                    /* WARNING: Subroutine does not return */
    FUN_ram_000011b0(&DAT_ram_000337e9,0xe,2);
  }
  if ((param_1 & 7) == 0) {
    return param_1;
  }
                    /* WARNING: Subroutine does not return */
  FUN_ram_000011b0(&DAT_ram_000337e9,0xe,0);
}

// Function: FUN_ram_00016cc0
void FUN_ram_00016cc0(undefined4 *param_1,longlong param_2)

{
  undefined4 uVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  
  uVar2 = *(ulonglong *)(param_2 + 0x18);
  if (*(ulonglong *)(param_2 + 0x10) != 0 || uVar2 != 0) {
    if (0xffffffffffff < uVar2) goto LAB_ram_00016e90;
    uVar1 = 0;
    if (*(ulonglong *)(param_2 + 0x30) < (*(ulonglong *)(param_2 + 0x10) >> 0x30 | uVar2 << 0x10))
    goto LAB_ram_00016d48;
  }
  uVar1 = 0;
  if (*(ulonglong *)(param_2 + 0x30) <= *(ulonglong *)(param_2 + 0x38)) {
    uVar4 = *(ulonglong *)(param_2 + 0x68);
    uVar3 = *(ulonglong *)(param_2 + 0x80);
    uVar2 = 0;
    if (*(ulonglong *)(param_2 + 0x60) != 0 || uVar4 != 0) {
      if (0xffffffffffff < uVar4) goto LAB_ram_00016e90;
      uVar2 = *(ulonglong *)(param_2 + 0x60) >> 0x30 | uVar4 << 0x10;
    }
    if (((*(ulonglong *)(param_2 + 0x38) <= uVar3) && (uVar2 <= uVar3)) &&
       (uVar3 <= *(ulonglong *)(param_2 + 0x88))) {
      uVar4 = *(ulonglong *)(param_2 + 0xb8);
      uVar3 = *(ulonglong *)(param_2 + 0xd0);
      uVar2 = 0;
      if (*(ulonglong *)(param_2 + 0xb0) != 0 || uVar4 != 0) {
        if (0xffffffffffff < uVar4) {
LAB_ram_00016e90:
                    /* WARNING: Subroutine does not return */
          FUN_ram_0002fb80(&DAT_ram_00034598);
        }
        uVar2 = *(ulonglong *)(param_2 + 0xb0) >> 0x30 | uVar4 << 0x10;
      }
      if (((*(ulonglong *)(param_2 + 0x88) <= uVar3) && (uVar2 <= uVar3)) &&
         (uVar3 <= *(ulonglong *)(param_2 + 0xd8))) {
        uVar1 = 0x1a;
      }
    }
  }
LAB_ram_00016d48:
  *param_1 = uVar1;
  param_1[1] = 0xbad2;
  return;
}

// Function: FUN_ram_00016ea8
longlong FUN_ram_00016ea8(longlong param_1,longlong param_2)

{
  longlong lVar1;
  
  if ((((param_1 < *(longlong *)(param_2 + 0x10)) ||
       (lVar1 = param_2, *(longlong *)(param_2 + 0x18) < param_1)) &&
      ((param_1 < *(longlong *)(param_2 + 0x40) ||
       (lVar1 = param_2 + 0x30, *(longlong *)(param_2 + 0x48) < param_1)))) &&
     ((param_1 < *(longlong *)(param_2 + 0x70) ||
      (lVar1 = param_2 + 0x60, *(longlong *)(param_2 + 0x78) < param_1)))) {
    lVar1 = 0;
  }
  return lVar1;
}

// Function: FUN_ram_00016f48
longlong FUN_ram_00016f48(ulonglong param_1,longlong param_2)

{
  longlong lVar1;
  
  if ((((param_1 < *(ulonglong *)(param_2 + 0x30)) ||
       (lVar1 = param_2, *(ulonglong *)(param_2 + 0x38) < param_1)) &&
      ((param_1 < *(ulonglong *)(param_2 + 0x80) ||
       (lVar1 = param_2 + 0x50, *(ulonglong *)(param_2 + 0x88) < param_1)))) &&
     ((param_1 < *(ulonglong *)(param_2 + 0xd0) ||
      (lVar1 = param_2 + 0xa0, *(ulonglong *)(param_2 + 0xd8) < param_1)))) {
    lVar1 = 0;
  }
  return lVar1;
}

// Function: FUN_ram_00016fe8
ulonglong FUN_ram_00016fe8(longlong param_1,ulonglong param_2,ulonglong param_3,ulonglong param_4,
                          longlong param_5)

{
  ulonglong uVar1;
  longlong lVar2;
  bool bVar3;
  ulonglong uVar4;
  bool bVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  longlong lVar9;
  ulonglong uVar10;
  ulonglong local_88;
  longlong local_80;
  ulonglong local_78;
  longlong local_70;
  ulonglong local_68;
  longlong local_60;
  ulonglong local_58;
  longlong local_50;
  longlong local_48;
  longlong local_40;
  ulonglong local_38;
  longlong local_30;
  longlong local_28;
  ulonglong local_20;
  longlong local_18;
  longlong local_10;
  ulonglong local_8;
  
  lVar9 = *(longlong *)(param_5 + -0x1000);
  FUN_ram_00031e70(&local_68,param_2,(longlong)param_2 >> 0x3f,lVar9);
  FUN_ram_00031e70(&local_58,lVar9,0,param_1,0);
  uVar6 = *(ulonglong *)(param_5 + -0xff8);
  FUN_ram_00031e70(&local_38,uVar6,(longlong)uVar6 >> 0x3f,param_1,param_1 >> 0x3f);
  FUN_ram_00031e70(&local_48,uVar6,(longlong)uVar6 >> 0x3f,param_2,(longlong)param_2 >> 0x3f);
  uVar4 = local_38 + local_68 + local_50;
  lVar2 = local_30 + (param_1 >> 0x3f & uVar6) + (ulonglong)(uVar4 < local_38);
  uVar6 = local_60 + (lVar9 >> 0x3f & param_2) + (ulonglong)(local_68 + local_50 < local_68);
  uVar7 = uVar6 + local_48;
  uVar1 = uVar7 + lVar2;
  uVar10 = 0;
  lVar2 = ((longlong)uVar6 >> 0x3f) + local_40 + (ulonglong)(uVar7 < uVar6) + (lVar2 >> 0x3f) +
          (ulonglong)(uVar1 < uVar7);
  uVar7 = (longlong)(uVar1 * 0x10000) >> 0x3f;
  uVar6 = 0xa1;
  if ((uVar1 >> 0x30 | lVar2 * 0x10000) != uVar7 || lVar2 >> 0x30 != uVar7) goto LAB_ram_00017968;
  uVar7 = uVar4 * 0x10000 | local_58 >> 0x30;
  uVar1 = uVar1 * 0x10000 | uVar4 >> 0x30;
  if (*(longlong *)(param_5 + -0xff0) == -1) {
    bVar3 = true;
    if (param_3 < uVar7) {
      if ((longlong)param_4 < (longlong)uVar1) goto LAB_ram_00017408;
LAB_ram_000173a8:
      bVar5 = false;
    }
    else {
      bVar3 = false;
      if ((longlong)uVar1 <= (longlong)param_4) goto LAB_ram_000173a8;
LAB_ram_00017408:
      bVar5 = true;
    }
    if (uVar1 != param_4) {
      bVar3 = bVar5;
    }
    uVar6 = 0;
    uVar10 = 0x1a;
    if (bVar3) goto LAB_ram_00017968;
  }
  else if (*(longlong *)(param_5 + -0xff0) == 1) {
    bVar3 = uVar7 < param_3;
    if ((longlong)uVar1 < (longlong)param_4) goto LAB_ram_00017408;
    goto LAB_ram_000173a8;
  }
  uVar4 = uVar7 + param_3;
  uVar10 = 0;
  uVar8 = uVar1 + param_4 + (ulonglong)(uVar4 < uVar7);
  uVar6 = 0xb7;
  if (-1 < (longlong)((uVar1 ^ param_4 ^ 0xffffffffffffffff) & (uVar1 ^ uVar8))) {
    if ((longlong)
        (((uVar8 & 0x7fffffffffff8000) << 1 | (uVar8 << 0x31 | uVar4 >> 0xf) >> 0x30) ^ uVar8) < 0)
    {
      uVar6 = 0xb9;
    }
    else {
      local_8 = (uVar1 - param_4) - (ulonglong)(uVar7 < param_3);
      if ((longlong)((uVar1 ^ param_4) & (uVar1 ^ local_8)) < 0) {
        uVar6 = 0xbb;
      }
      else {
        lVar2 = uVar7 - param_3;
        local_10 = -lVar2;
        if (-1 < (longlong)local_8) {
          local_10 = lVar2;
        }
        if ((longlong)local_8 < 0) {
          local_8 = -(local_8 + (lVar2 != 0));
        }
        FUN_ram_00001708(&local_28,&local_10,(uVar4 >> 0xf) << 0x10 | uVar4 * 2 & 0xfffe);
        uVar6 = 0xbe;
        if (local_28 != 0) {
          FUN_ram_00031e70(&local_78,local_18,local_18 >> 0x3f,0x86a0000000000000,0xffffffffffffffff
                          );
          FUN_ram_00031e70(&local_88,local_20,0,0x86a0000000000000,0);
          uVar1 = local_20 + local_78 + local_80;
          uVar6 = local_70 + local_18 + (ulonglong)(local_78 + local_80 < local_78);
          uVar4 = uVar6 + local_18;
          uVar7 = uVar4 + (uVar1 < local_20);
          lVar2 = ((longlong)uVar6 >> 0x3f) + (local_18 >> 0x3f) + (ulonglong)(uVar4 < uVar6) +
                  (ulonglong)(uVar7 < uVar4);
          uVar4 = (longlong)(uVar7 * 0x10000) >> 0x3f;
          uVar6 = 0xc1;
          if ((uVar7 >> 0x30 | lVar2 * 0x10000) == uVar4 && lVar2 >> 0x30 == uVar4) {
            uVar6 = uVar7 * 0x10000 | uVar1 >> 0x30;
            uVar10 = 0x1a;
            if ((uVar1 * 0x10000 == 0 && (local_88 & 0xffe0000000000000) == 0) && uVar6 == 0) {
              uVar6 = 0;
            }
            else if (uVar6 < 0x10000) {
              uVar6 = uVar1 * 0x10000 >> 0x30 | uVar6 << 0x10;
            }
            else {
              uVar10 = 0;
              uVar6 = 0xc3;
            }
          }
        }
      }
    }
  }
LAB_ram_00017968:
  return uVar6 << 0x20 | uVar10;
}

// Function: FUN_ram_00017980
void FUN_ram_00017980(undefined8 *param_1,ulonglong param_2,uint *param_3)

{
  longlong lVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  uint *puVar5;
  uint *puVar6;
  ulonglong uVar7;
  longlong local_28;
  undefined8 local_20;
  undefined8 local_18;
  longlong local_10;
  ulonglong local_8;
  
  if (50000 < (param_2 & 0xffffffff)) {
    *(undefined4 *)(param_1 + 1) = 0x31;
    goto LAB_ram_00017bb8;
  }
  uVar4 = 0;
  uVar2 = (ulonglong)*param_3;
  if (uVar2 <= (param_2 & 0xffffffff)) {
    uVar4 = (ulonglong)param_3[2];
    if (uVar4 == uVar2) {
      uVar4 = (ulonglong)param_3[1];
    }
    else {
      lVar1 = 8;
      puVar6 = param_3;
      if (uVar4 <= (param_2 & 0xffffffff)) {
        puVar5 = param_3 + 2;
        uVar3 = (ulonglong)param_3[4];
        if (uVar3 == uVar4) {
LAB_ram_00017a58:
          uVar4 = (ulonglong)puVar5[1];
          goto LAB_ram_00017af0;
        }
        lVar1 = 0x10;
        uVar2 = uVar4;
        uVar4 = uVar3;
        puVar6 = puVar5;
        if (uVar3 <= (param_2 & 0xffffffff)) {
          puVar5 = param_3 + 4;
          uVar7 = (ulonglong)param_3[6];
          if (uVar7 == uVar3) goto LAB_ram_00017a58;
          lVar1 = 0x18;
          uVar2 = uVar3;
          uVar4 = uVar7;
          puVar6 = puVar5;
          if (uVar7 <= (param_2 & 0xffffffff)) {
            puVar5 = param_3 + 6;
            uVar4 = (ulonglong)param_3[8];
            if (uVar4 == uVar7) goto LAB_ram_00017a58;
            lVar1 = 0x20;
            uVar2 = uVar7;
            puVar6 = puVar5;
            if (uVar4 <= (param_2 & 0xffffffff)) {
              puVar5 = param_3 + 8;
              uVar3 = (ulonglong)param_3[10];
              if (uVar3 == uVar4) goto LAB_ram_00017a58;
              lVar1 = 0x28;
              uVar2 = uVar4;
              uVar4 = uVar3;
              puVar6 = puVar5;
              if (uVar3 <= (param_2 & 0xffffffff)) {
                puVar5 = param_3 + 10;
                uVar4 = (ulonglong)param_3[0xc];
                if (uVar4 == uVar3) goto LAB_ram_00017a58;
                lVar1 = 0x30;
                uVar2 = uVar3;
                puVar6 = puVar5;
                if (uVar4 <= (param_2 & 0xffffffff)) {
                  puVar5 = param_3 + 0xc;
                  uVar3 = (ulonglong)param_3[0xe];
                  if (uVar3 == uVar4) goto LAB_ram_00017a58;
                  lVar1 = 0x38;
                  uVar2 = uVar4;
                  uVar4 = uVar3;
                  puVar6 = puVar5;
                  if (uVar3 <= (param_2 & 0xffffffff)) {
                    puVar5 = param_3 + 0xe;
                    uVar4 = (ulonglong)param_3[0x10];
                    if (uVar4 == uVar3) goto LAB_ram_00017a58;
                    lVar1 = 0x40;
                    uVar2 = uVar3;
                    puVar6 = puVar5;
                    if (uVar4 <= (param_2 & 0xffffffff)) {
                      uVar3 = (ulonglong)param_3[0x12];
                      if (uVar3 == uVar4) {
                        uVar4 = (ulonglong)param_3[0x11];
                        goto LAB_ram_00017af0;
                      }
                      puVar5 = param_3 + 0x12;
                      lVar1 = 0x48;
                      uVar2 = uVar4;
                      uVar4 = uVar3;
                      puVar6 = param_3 + 0x10;
                      if (uVar3 <= (param_2 & 0xffffffff)) goto LAB_ram_00017a58;
                    }
                  }
                }
              }
            }
          }
        }
      }
      uVar4 = (ulonglong)puVar6[1] +
              (((ulonglong)*(uint *)((longlong)param_3 + lVar1 + 4) - (ulonglong)puVar6[1]) *
              ((param_2 & 0xffffffff) - uVar2)) / (uVar4 - uVar2);
    }
  }
LAB_ram_00017af0:
  local_10 = uVar4 << 0x30;
  local_8 = (uVar4 & 0xffff0000) >> 0x10;
  FUN_ram_00001708(&local_28,&local_10,0xa000000000000,0);
  if (local_28 != 0) {
    param_1[2] = local_18;
    param_1[1] = local_20;
    *(undefined4 *)param_1 = 0;
    return;
  }
  *(undefined4 *)(param_1 + 1) = 0xcf;
LAB_ram_00017bb8:
  *param_1 = 1;
  return;
}

// Function: FUN_ram_00017df8
void FUN_ram_00017df8(undefined4 *param_1,ulonglong param_2,ulonglong param_3,undefined8 param_4,
                     longlong param_5)

{
  ulonglong uVar1;
  
  uVar1 = FUN_ram_00016fe8(param_2 << 0x30,((longlong)param_2 >> 0x3f) << 0x30 | param_2 >> 0x10,
                           param_3 << 0x30,((longlong)param_3 >> 0x3f) << 0x30 | param_3 >> 0x10);
  if ((uVar1 & 0xffffffff) == 0x1a) {
    FUN_ram_00017980(param_1,uVar1 >> 0x20,*(undefined8 *)(param_5 + -0xff8));
  }
  else {
    param_1[2] = (int)(uVar1 >> 0x20);
    param_1[1] = (int)uVar1;
    *param_1 = 1;
  }
  return;
}

// Function: FUN_ram_00017f28
/* WARNING: Type propagation algorithm not settling */

void FUN_ram_00017f28(undefined4 *param_1,ulonglong param_2,ulonglong param_3,ulonglong param_4,
                     longlong param_5)

{
  byte bVar1;
  undefined *puVar2;
  ulonglong uVar3;
  undefined *puVar4;
  bool bVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  bool bVar8;
  longlong lVar9;
  undefined *puVar10;
  longlong *plVar11;
  ulonglong uVar12;
  undefined *puVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  undefined *puVar17;
  undefined *puVar18;
  ulonglong uVar19;
  undefined *puVar20;
  ulonglong uVar21;
  longlong lVar22;
  ulonglong local_200;
  longlong local_1f8;
  undefined *local_1d8;
  undefined *local_1d0;
  undefined *local_1c0;
  undefined *local_1b8;
  ulonglong local_1b0;
  longlong local_1a8;
  ulonglong local_1a0;
  longlong local_198;
  longlong local_190;
  longlong local_188;
  ulonglong local_180;
  longlong local_178;
  undefined *local_170;
  ulonglong local_168;
  ulonglong local_160;
  longlong local_158;
  ulonglong local_150;
  longlong local_148;
  longlong local_140;
  longlong local_138;
  ulonglong local_130;
  longlong local_128;
  undefined *local_120;
  ulonglong local_118;
  ulonglong local_110;
  longlong local_108;
  ulonglong local_100;
  longlong local_f8;
  longlong local_f0;
  longlong local_e8;
  undefined *local_e0;
  ulonglong local_d8;
  ulonglong local_d0;
  longlong local_c8;
  ulonglong local_c0;
  longlong local_b8;
  longlong local_b0;
  longlong local_a8;
  ulonglong local_a0;
  longlong local_98;
  undefined *local_90;
  ulonglong local_88;
  ulonglong local_80;
  longlong local_78;
  ulonglong local_70;
  longlong local_68;
  longlong local_60;
  longlong local_58;
  undefined *local_50;
  undefined *local_48;
  ulonglong local_40;
  undefined8 local_38;
  undefined8 local_30;
  longlong local_20;
  undefined *local_18;
  undefined *local_10;
  undefined *local_8;
  
  local_10 = (undefined *)(param_2 << 0x30);
  local_8 = (undefined *)(((longlong)param_2 >> 0x3f) << 0x30 | param_2 >> 0x10);
  puVar20 = *(undefined **)(param_5 + -0x1000);
  uVar15 = *(ulonglong *)(param_5 + -0xff8);
  FUN_ram_00001708(&local_50,&local_10,puVar20,uVar15);
  uVar19 = local_40;
  puVar17 = local_48;
  if (local_50 == (undefined *)0x1) {
    if (puVar20 == (undefined *)0x0 && uVar15 == 0) {
LAB_ram_00018448:
      uVar7 = 0x9700000000;
      goto LAB_ram_00018460;
    }
    uVar16 = ((longlong)param_3 >> 0x3f) << 0x30 | param_3 >> 0x10;
    local_20 = param_3 << 0x30;
    lVar9 = -local_20;
    if ((longlong)uVar16 < 0) {
      bVar5 = local_20 != 0;
      local_20 = lVar9;
      if (bVar5) goto LAB_ram_00018078;
LAB_ram_000181f0:
      lVar9 = 0;
    }
    else {
      if (local_20 == 0) goto LAB_ram_000181f0;
LAB_ram_00018078:
      lVar9 = 1;
    }
    local_18 = (undefined *)uVar16;
    if ((longlong)uVar16 < 0) {
      local_18 = (undefined *)-(uVar16 + lVar9);
    }
    plVar11 = *(longlong **)(param_5 + -0xff0);
    bVar5 = puVar20 != (undefined *)0x0;
    uVar21 = uVar15;
    if ((longlong)uVar15 < 0) {
      puVar20 = (undefined *)-(longlong)puVar20;
      uVar21 = -(uVar15 + bVar5);
    }
    local_10 = puVar20;
    local_8 = (undefined *)uVar21;
    FUN_ram_00001298(&local_50,&local_20,&local_10,0x30);
    if ((char)local_40 != '\0') goto LAB_ram_00018448;
    if ((longlong)(uVar16 ^ uVar15) < 0) {
      bVar8 = true;
      bVar5 = true;
      if (local_50 == (undefined *)0x0) {
        bVar5 = false;
        if (local_48 < (undefined *)0x8000000000000001) goto LAB_ram_000182c8;
LAB_ram_00018288:
        if (local_48 == (undefined *)0x8000000000000000) goto LAB_ram_00018290;
LAB_ram_000182d8:
        if (bVar8) goto LAB_ram_00018448;
      }
      else {
        if ((undefined *)0x8000000000000000 < local_48) goto LAB_ram_00018288;
LAB_ram_000182c8:
        bVar8 = false;
        if (local_48 != (undefined *)0x8000000000000000) goto LAB_ram_000182d8;
LAB_ram_00018290:
        if (bVar5) goto LAB_ram_00018448;
      }
      local_1c0 = (undefined *)-(longlong)local_50;
      local_1d0 = (undefined *)((ulonglong)local_48 ^ 0xffffffffffffffff);
      if (local_1c0 == (undefined *)0x0) {
        local_1d0 = (undefined *)-(longlong)local_48;
      }
    }
    else {
      local_1d0 = local_48;
      local_1c0 = local_50;
      if ((longlong)local_48 < 0) goto LAB_ram_00018448;
    }
    uVar16 = ((longlong)param_4 >> 0x3f) << 0x30 | param_4 >> 0x10;
    local_20 = param_4 << 0x30;
    lVar9 = -local_20;
    if ((longlong)uVar16 < 0) {
      bVar5 = local_20 != 0;
      local_20 = lVar9;
      if (bVar5) goto LAB_ram_000183b0;
LAB_ram_000184a0:
      lVar9 = 0;
    }
    else {
      if (local_20 == 0) goto LAB_ram_000184a0;
LAB_ram_000183b0:
      lVar9 = 1;
    }
    local_18 = (undefined *)uVar16;
    if ((longlong)uVar16 < 0) {
      local_18 = (undefined *)-(uVar16 + lVar9);
    }
    local_10 = puVar20;
    local_8 = (undefined *)uVar21;
    FUN_ram_00001298(&local_50,&local_20,&local_10,0x30);
    if ((char)local_40 != '\0') goto LAB_ram_00018448;
    if ((longlong)(uVar16 ^ uVar15) < 0) {
      bVar8 = true;
      bVar5 = true;
      if (local_50 == (undefined *)0x0) {
        bVar5 = false;
        if (local_48 < (undefined *)0x8000000000000001) goto LAB_ram_00018570;
LAB_ram_00018530:
        if (local_48 == (undefined *)0x8000000000000000) goto LAB_ram_00018538;
LAB_ram_00018580:
        if (bVar8) goto LAB_ram_00018448;
      }
      else {
        if ((undefined *)0x8000000000000000 < local_48) goto LAB_ram_00018530;
LAB_ram_00018570:
        bVar8 = false;
        if (local_48 != (undefined *)0x8000000000000000) goto LAB_ram_00018580;
LAB_ram_00018538:
        if (bVar5) goto LAB_ram_00018448;
      }
      puVar20 = (undefined *)-(longlong)local_50;
      puVar10 = (undefined *)((ulonglong)local_48 ^ 0xffffffffffffffff);
      if (puVar20 == (undefined *)0x0) {
        puVar10 = (undefined *)-(longlong)local_48;
      }
    }
    else {
      puVar20 = local_50;
      puVar10 = local_48;
      if ((longlong)local_48 < 0) goto LAB_ram_00018448;
    }
    bVar1 = *(byte *)(plVar11 + 4);
    if (3 < bVar1) goto LAB_ram_0001b338;
    uVar15 = (longlong)local_1c0 >> 0x3f;
    if (bVar1 < 2) {
      if (bVar1 == 0) {
        if (-1 < (longlong)uVar19) {
          if (puVar17 != (undefined *)0x0 || uVar19 != 0) {
            if (uVar19 == 0) {
              uVar16 = (ulonglong)puVar17 | (ulonglong)puVar17 >> 1;
              uVar16 = uVar16 | uVar16 >> 2;
              uVar16 = uVar16 | uVar16 >> 4;
              uVar16 = uVar16 | uVar16 >> 8;
              uVar16 = uVar16 | uVar16 >> 0x10;
              uVar16 = (uVar16 | uVar16 >> 0x20) ^ 0xffffffffffffffff;
              uVar16 = uVar16 - (uVar16 >> 1 & 0x5555555555555555);
              uVar16 = (uVar16 & 0x3333333333333333) + (uVar16 >> 2 & 0x3333333333333333);
              uVar16 = ((uVar16 + (uVar16 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38) +
                       0x40;
            }
            else {
              uVar16 = uVar19 | uVar19 >> 1;
              uVar16 = uVar16 | uVar16 >> 2;
              uVar16 = uVar16 | uVar16 >> 4;
              uVar16 = uVar16 | uVar16 >> 8;
              uVar16 = uVar16 | uVar16 >> 0x10;
              uVar16 = (uVar16 | uVar16 >> 0x20) ^ 0xffffffffffffffff;
              uVar16 = uVar16 - (uVar16 >> 1 & 0x5555555555555555);
              uVar16 = (uVar16 & 0x3333333333333333) + (uVar16 >> 2 & 0x3333333333333333);
              uVar16 = (uVar16 + (uVar16 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38;
            }
            puVar13 = (undefined *)0x0;
            FUN_ram_00031e28(&local_e0,1,0,(uVar16 ^ 0xffffffffffffffff) & 0x7e);
            uVar16 = 0;
            do {
              puVar2 = local_e0 + (longlong)puVar13;
              uVar21 = local_d8 + uVar16 + (ulonglong)(puVar2 < local_e0);
              bVar5 = true;
              if (uVar19 < uVar21) {
                if (puVar17 < puVar2) goto LAB_ram_00018ee0;
LAB_ram_00019018:
                bVar8 = false;
                if (uVar19 != uVar21) goto LAB_ram_00019028;
LAB_ram_00018ee8:
                if (bVar8) goto LAB_ram_00018ef8;
LAB_ram_00019040:
                uVar12 = local_d8;
                if (!bVar8) goto LAB_ram_00019058;
LAB_ram_00018f08:
                uVar21 = 0;
                if (bVar8) {
LAB_ram_00018f18:
                  puVar2 = (undefined *)0x0;
                }
              }
              else {
                bVar5 = false;
                if (puVar2 <= puVar17) goto LAB_ram_00019018;
LAB_ram_00018ee0:
                bVar8 = true;
                if (uVar19 == uVar21) goto LAB_ram_00018ee8;
LAB_ram_00019028:
                bVar8 = bVar5;
                if (!bVar8) goto LAB_ram_00019040;
LAB_ram_00018ef8:
                uVar12 = 0;
                if (bVar8) goto LAB_ram_00018f08;
LAB_ram_00019058:
                if (bVar8) goto LAB_ram_00018f18;
              }
              puVar4 = (undefined *)0x0;
              if (!bVar8) {
                puVar4 = local_e0;
              }
              puVar13 = puVar4 + ((ulonglong)puVar13 >> 1 | uVar16 << 0x3f);
              lVar9 = 1;
              if (puVar13 < puVar4) {
                if (puVar17 < puVar2) goto LAB_ram_00018f98;
LAB_ram_000190b0:
                lVar22 = 0;
                if (local_e0 < (undefined *)0x4) goto LAB_ram_000190c8;
LAB_ram_00018fa8:
                bVar5 = true;
              }
              else {
                lVar9 = 0;
                if (puVar2 <= puVar17) goto LAB_ram_000190b0;
LAB_ram_00018f98:
                lVar22 = 1;
                if ((undefined *)0x3 < local_e0) goto LAB_ram_00018fa8;
LAB_ram_000190c8:
                bVar5 = false;
              }
              if (local_d8 != 0) {
                bVar5 = local_d8 != 0;
              }
              uVar19 = (uVar19 - uVar21) - lVar22;
              uVar16 = uVar12 + (uVar16 >> 1) + lVar9;
              puVar17 = puVar17 + -(longlong)puVar2;
              local_e0 = (undefined *)((ulonglong)local_e0 >> 2 | local_d8 << 0x3e);
              local_d8 = local_d8 >> 2;
            } while (bVar5);
            local_200 = uVar16 * 0x1000000 | (ulonglong)puVar13 >> 0x28;
            puVar17 = (undefined *)((longlong)puVar13 * 0x1000000);
            goto LAB_ram_00019960;
          }
          puVar17 = (undefined *)0x0;
          local_200 = 0;
LAB_ram_00019978:
          if (-1 < (longlong)local_1d0) {
            if (local_1c0 == (undefined *)0x0 && local_1d0 == (undefined *)0x0) {
              local_1c0 = (undefined *)0x0;
              local_1d0 = (undefined *)0x0;
            }
            else {
              if (local_1d0 == (undefined *)0x0) {
                uVar19 = (ulonglong)local_1c0 | (ulonglong)local_1c0 >> 1;
                uVar19 = uVar19 | uVar19 >> 2;
                uVar19 = uVar19 | uVar19 >> 4;
                uVar19 = uVar19 | uVar19 >> 8;
                uVar19 = uVar19 | uVar19 >> 0x10;
                uVar19 = (uVar19 | uVar19 >> 0x20) ^ 0xffffffffffffffff;
                uVar19 = uVar19 - (uVar19 >> 1 & 0x5555555555555555);
                uVar19 = (uVar19 & 0x3333333333333333) + (uVar19 >> 2 & 0x3333333333333333);
                uVar19 = ((uVar19 + (uVar19 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38)
                         + 0x40;
              }
              else {
                uVar19 = (ulonglong)local_1d0 | (ulonglong)local_1d0 >> 1;
                uVar19 = uVar19 | uVar19 >> 2;
                uVar19 = uVar19 | uVar19 >> 4;
                uVar19 = uVar19 | uVar19 >> 8;
                uVar19 = uVar19 | uVar19 >> 0x10;
                uVar19 = (uVar19 | uVar19 >> 0x20) ^ 0xffffffffffffffff;
                uVar19 = uVar19 - (uVar19 >> 1 & 0x5555555555555555);
                uVar19 = (uVar19 & 0x3333333333333333) + (uVar19 >> 2 & 0x3333333333333333);
                uVar19 = (uVar19 + (uVar19 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38;
              }
              puVar13 = (undefined *)0x0;
              FUN_ram_00031e28(&local_170,1,0,(uVar19 ^ 0xffffffffffffffff) & 0x7e);
              uVar19 = 0;
              do {
                puVar2 = local_170 + (longlong)puVar13;
                puVar4 = (undefined *)(local_168 + uVar19 + (ulonglong)(puVar2 < local_170));
                bVar5 = true;
                if (local_1d0 < puVar4) {
                  if (local_1c0 < puVar2) goto LAB_ram_0001a1f8;
LAB_ram_0001a330:
                  bVar8 = false;
                  if (local_1d0 != puVar4) goto LAB_ram_0001a348;
LAB_ram_0001a208:
                  if (bVar8) goto LAB_ram_0001a218;
LAB_ram_0001a360:
                  uVar15 = local_168;
                  if (!bVar8) goto LAB_ram_0001a378;
LAB_ram_0001a228:
                  puVar4 = (undefined *)0x0;
                  if (bVar8) {
LAB_ram_0001a238:
                    puVar2 = (undefined *)0x0;
                  }
                }
                else {
                  bVar5 = false;
                  if (puVar2 <= local_1c0) goto LAB_ram_0001a330;
LAB_ram_0001a1f8:
                  bVar8 = true;
                  if (local_1d0 == puVar4) goto LAB_ram_0001a208;
LAB_ram_0001a348:
                  bVar8 = bVar5;
                  if (!bVar8) goto LAB_ram_0001a360;
LAB_ram_0001a218:
                  uVar15 = 0;
                  if (bVar8) goto LAB_ram_0001a228;
LAB_ram_0001a378:
                  if (bVar8) goto LAB_ram_0001a238;
                }
                puVar18 = (undefined *)0x0;
                if (!bVar8) {
                  puVar18 = local_170;
                }
                puVar13 = puVar18 + ((ulonglong)puVar13 >> 1 | uVar19 << 0x3f);
                lVar9 = 1;
                if (puVar13 < puVar18) {
                  if (local_1c0 < puVar2) goto LAB_ram_0001a2b0;
LAB_ram_0001a3d0:
                  lVar22 = 0;
                  if (local_170 < (undefined *)0x4) goto LAB_ram_0001a3e8;
LAB_ram_0001a2c0:
                  bVar5 = true;
                }
                else {
                  lVar9 = 0;
                  if (puVar2 <= local_1c0) goto LAB_ram_0001a3d0;
LAB_ram_0001a2b0:
                  lVar22 = 1;
                  if ((undefined *)0x3 < local_170) goto LAB_ram_0001a2c0;
LAB_ram_0001a3e8:
                  bVar5 = false;
                }
                if (local_168 != 0) {
                  bVar5 = local_168 != 0;
                }
                local_1d0 = local_1d0 + (-(longlong)puVar4 - lVar22);
                uVar19 = uVar15 + (uVar19 >> 1) + lVar9;
                local_1c0 = local_1c0 + -(longlong)puVar2;
                local_170 = (undefined *)((ulonglong)local_170 >> 2 | local_168 << 0x3e);
                local_168 = local_168 >> 2;
              } while (bVar5);
              local_1d0 = (undefined *)(uVar19 * 0x1000000 | (ulonglong)puVar13 >> 0x28);
              local_1c0 = (undefined *)((longlong)puVar13 * 0x1000000);
            }
            goto LAB_ram_0001ac38;
          }
        }
LAB_ram_0001b308:
        FUN_ram_0002fbd8(&DAT_ram_0003378f,0x2c,&DAT_ram_000340f8);
LAB_ram_0001b338:
        local_50 = &DAT_ram_00034258;
        local_30 = 0;
        local_48 = (undefined *)0x1;
        local_38 = 0;
        local_40 = 8;
                    /* WARNING: Subroutine does not return */
        FUN_ram_0002fba8(&local_50,&DAT_ram_00034268);
      }
      local_200 = uVar19;
    }
    else {
      uVar16 = (longlong)puVar17 >> 0x3f;
      if (bVar1 == 2) {
        if ((longlong)uVar19 < 0) goto LAB_ram_0001b308;
        if (puVar17 == (undefined *)0x0 && uVar19 == 0) {
          lVar9 = 0;
          uVar21 = 0;
        }
        else {
          if (uVar19 == 0) {
            uVar21 = (ulonglong)puVar17 | (ulonglong)puVar17 >> 1;
            uVar21 = uVar21 | uVar21 >> 2;
            uVar21 = uVar21 | uVar21 >> 4;
            uVar21 = uVar21 | uVar21 >> 8;
            uVar21 = uVar21 | uVar21 >> 0x10;
            uVar21 = (uVar21 | uVar21 >> 0x20) ^ 0xffffffffffffffff;
            uVar21 = uVar21 - (uVar21 >> 1 & 0x5555555555555555);
            uVar21 = (uVar21 & 0x3333333333333333) + (uVar21 >> 2 & 0x3333333333333333);
            uVar21 = ((uVar21 + (uVar21 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38) +
                     0x40;
          }
          else {
            uVar21 = uVar19 | uVar19 >> 1;
            uVar21 = uVar21 | uVar21 >> 2;
            uVar21 = uVar21 | uVar21 >> 4;
            uVar21 = uVar21 | uVar21 >> 8;
            uVar21 = uVar21 | uVar21 >> 0x10;
            uVar21 = (uVar21 | uVar21 >> 0x20) ^ 0xffffffffffffffff;
            uVar21 = uVar21 - (uVar21 >> 1 & 0x5555555555555555);
            uVar21 = (uVar21 & 0x3333333333333333) + (uVar21 >> 2 & 0x3333333333333333);
            uVar21 = (uVar21 + (uVar21 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38;
          }
          puVar13 = (undefined *)0x0;
          FUN_ram_00031e28(&local_90,1,0,(uVar21 ^ 0xffffffffffffffff) & 0x7e);
          uVar12 = 0;
          local_1d8 = puVar17;
          uVar21 = uVar19;
          do {
            puVar2 = local_90 + (longlong)puVar13;
            uVar3 = local_88 + uVar12 + (ulonglong)(puVar2 < local_90);
            bVar5 = true;
            if (uVar21 < uVar3) {
              if (local_1d8 < puVar2) goto LAB_ram_000193a0;
LAB_ram_000194d0:
              bVar8 = false;
              if (uVar21 != uVar3) goto LAB_ram_000194e0;
LAB_ram_000193a8:
              if (bVar8) goto LAB_ram_000193b8;
LAB_ram_000194f8:
              uVar14 = local_88;
              if (!bVar8) goto LAB_ram_00019510;
LAB_ram_000193c8:
              uVar3 = 0;
              if (bVar8) {
LAB_ram_000193d8:
                puVar2 = (undefined *)0x0;
              }
            }
            else {
              bVar5 = false;
              if (puVar2 <= local_1d8) goto LAB_ram_000194d0;
LAB_ram_000193a0:
              bVar8 = true;
              if (uVar21 == uVar3) goto LAB_ram_000193a8;
LAB_ram_000194e0:
              bVar8 = bVar5;
              if (!bVar8) goto LAB_ram_000194f8;
LAB_ram_000193b8:
              uVar14 = 0;
              if (bVar8) goto LAB_ram_000193c8;
LAB_ram_00019510:
              if (bVar8) goto LAB_ram_000193d8;
            }
            puVar4 = (undefined *)0x0;
            if (!bVar8) {
              puVar4 = local_90;
            }
            puVar13 = puVar4 + ((ulonglong)puVar13 >> 1 | uVar12 << 0x3f);
            if (puVar13 < puVar4) {
              local_200 = 1;
              if (local_1d8 < puVar2) goto LAB_ram_00019460;
LAB_ram_00019578:
              lVar9 = 0;
              if (local_90 < (undefined *)0x4) goto LAB_ram_00019590;
LAB_ram_00019470:
              bVar5 = true;
              if (local_88 != 0) goto LAB_ram_00019488;
LAB_ram_000195b0:
              bVar8 = false;
            }
            else {
              local_200 = 0;
              if (puVar2 <= local_1d8) goto LAB_ram_00019578;
LAB_ram_00019460:
              lVar9 = 1;
              if ((undefined *)0x3 < local_90) goto LAB_ram_00019470;
LAB_ram_00019590:
              bVar5 = false;
              if (local_88 == 0) goto LAB_ram_000195b0;
LAB_ram_00019488:
              bVar8 = true;
            }
            if (local_88 != 0) {
              bVar5 = bVar8;
            }
            uVar21 = (uVar21 - uVar3) - lVar9;
            uVar12 = uVar14 + (uVar12 >> 1) + local_200;
            local_1d8 = local_1d8 + -(longlong)puVar2;
            local_90 = (undefined *)((ulonglong)local_90 >> 2 | local_88 << 0x3e);
            local_88 = local_88 >> 2;
          } while (bVar5);
          uVar21 = uVar12 * 0x1000000 | (ulonglong)puVar13 >> 0x28;
          lVar9 = (longlong)puVar13 * 0x1000000;
        }
        FUN_ram_00031e70(&local_d0,uVar19,0);
        FUN_ram_00031e70(&local_c0,lVar9,0,puVar17,0);
        FUN_ram_00031e70(&local_a0,uVar21,(longlong)uVar21 >> 0x3f,puVar17,uVar16);
        FUN_ram_00031e70(&local_b0,uVar21,(longlong)uVar21 >> 0x3f,uVar19,0);
        uVar12 = local_a0 + local_d0 + local_b8;
        lVar22 = local_98 + (uVar16 & uVar21) + (ulonglong)(uVar12 < local_a0);
        uVar19 = local_c8 + (uVar19 & lVar9 >> 0x3f & 0x7fffffffffffffff) +
                 (ulonglong)(local_d0 + local_b8 < local_d0);
        uVar16 = uVar19 + local_b0;
        uVar21 = uVar16 + lVar22;
        lVar9 = ((longlong)uVar19 >> 0x3f) + local_a8 + (ulonglong)(uVar16 < uVar19) +
                (lVar22 >> 0x3f) + (ulonglong)(uVar21 < uVar16);
        uVar19 = (longlong)(uVar21 * 0x10000) >> 0x3f;
        if ((uVar21 >> 0x30 | lVar9 * 0x10000) != uVar19 || lVar9 >> 0x30 != uVar19)
        goto LAB_ram_0001b3b0;
        local_200 = uVar21 * 0x10000 | uVar12 >> 0x30;
        puVar17 = (undefined *)(uVar12 * 0x10000 | local_c0 >> 0x30);
LAB_ram_00019960:
        if (bVar1 < 2) {
          if (bVar1 == 0) goto LAB_ram_00019978;
          goto LAB_ram_0001ac38;
        }
        local_1d8 = puVar17;
        if (bVar1 != 2) goto LAB_ram_00019ce8;
        if ((longlong)local_1d0 < 0) goto LAB_ram_0001b308;
        if (local_1c0 == (undefined *)0x0 && local_1d0 == (undefined *)0x0) {
          lVar9 = 0;
          uVar19 = 0;
        }
        else {
          if (local_1d0 == (undefined *)0x0) {
            uVar19 = (ulonglong)local_1c0 | (ulonglong)local_1c0 >> 1;
            uVar19 = uVar19 | uVar19 >> 2;
            uVar19 = uVar19 | uVar19 >> 4;
            uVar19 = uVar19 | uVar19 >> 8;
            uVar19 = uVar19 | uVar19 >> 0x10;
            uVar19 = (uVar19 | uVar19 >> 0x20) ^ 0xffffffffffffffff;
            uVar19 = uVar19 - (uVar19 >> 1 & 0x5555555555555555);
            uVar19 = (uVar19 & 0x3333333333333333) + (uVar19 >> 2 & 0x3333333333333333);
            uVar19 = ((uVar19 + (uVar19 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38) +
                     0x40;
          }
          else {
            uVar19 = (ulonglong)local_1d0 | (ulonglong)local_1d0 >> 1;
            uVar19 = uVar19 | uVar19 >> 2;
            uVar19 = uVar19 | uVar19 >> 4;
            uVar19 = uVar19 | uVar19 >> 8;
            uVar19 = uVar19 | uVar19 >> 0x10;
            uVar19 = (uVar19 | uVar19 >> 0x20) ^ 0xffffffffffffffff;
            uVar19 = uVar19 - (uVar19 >> 1 & 0x5555555555555555);
            uVar19 = (uVar19 & 0x3333333333333333) + (uVar19 >> 2 & 0x3333333333333333);
            uVar19 = (uVar19 + (uVar19 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38;
          }
          puVar2 = (undefined *)0x0;
          FUN_ram_00031e28(&local_120,1,0,(uVar19 ^ 0xffffffffffffffff) & 0x7e);
          uVar19 = 0;
          puVar17 = local_1c0;
          puVar13 = local_1d0;
          do {
            puVar4 = local_120 + (longlong)puVar2;
            local_1b8 = (undefined *)(local_118 + uVar19 + (ulonglong)(puVar4 < local_120));
            bVar5 = true;
            if (puVar13 < local_1b8) {
              if (puVar17 < puVar4) goto LAB_ram_0001a6b0;
LAB_ram_0001a7f0:
              bVar8 = false;
              if (puVar13 != local_1b8) goto LAB_ram_0001a800;
LAB_ram_0001a6b8:
              if (bVar8) goto LAB_ram_0001a6d0;
LAB_ram_0001a820:
              uVar16 = local_118;
              if (!bVar8) goto LAB_ram_0001a840;
LAB_ram_0001a6e8:
              local_1b8 = (undefined *)0x0;
              if (bVar8) {
LAB_ram_0001a700:
                puVar4 = (undefined *)0x0;
              }
            }
            else {
              bVar5 = false;
              if (puVar4 <= puVar17) goto LAB_ram_0001a7f0;
LAB_ram_0001a6b0:
              bVar8 = true;
              if (puVar13 == local_1b8) goto LAB_ram_0001a6b8;
LAB_ram_0001a800:
              bVar8 = bVar5;
              if (!bVar8) goto LAB_ram_0001a820;
LAB_ram_0001a6d0:
              uVar16 = 0;
              if (bVar8) goto LAB_ram_0001a6e8;
LAB_ram_0001a840:
              if (bVar8) goto LAB_ram_0001a700;
            }
            puVar18 = (undefined *)0x0;
            if (!bVar8) {
              puVar18 = local_120;
            }
            puVar2 = puVar18 + ((ulonglong)puVar2 >> 1 | uVar19 << 0x3f);
            if (puVar2 < puVar18) {
              local_1f8 = 1;
              if (puVar17 < puVar4) goto LAB_ram_0001a778;
LAB_ram_0001a8a0:
              lVar9 = 0;
              if (local_120 < (undefined *)0x4) goto LAB_ram_0001a8b8;
LAB_ram_0001a788:
              bVar5 = true;
            }
            else {
              local_1f8 = 0;
              if (puVar4 <= puVar17) goto LAB_ram_0001a8a0;
LAB_ram_0001a778:
              lVar9 = 1;
              if ((undefined *)0x3 < local_120) goto LAB_ram_0001a788;
LAB_ram_0001a8b8:
              bVar5 = false;
            }
            if (local_118 != 0) {
              bVar5 = local_118 != 0;
            }
            puVar13 = puVar13 + (-(longlong)local_1b8 - lVar9);
            uVar19 = uVar16 + (uVar19 >> 1) + local_1f8;
            puVar17 = puVar17 + -(longlong)puVar4;
            local_120 = (undefined *)((ulonglong)local_120 >> 2 | local_118 << 0x3e);
            local_118 = local_118 >> 2;
          } while (bVar5);
          uVar19 = uVar19 * 0x1000000 | (ulonglong)puVar2 >> 0x28;
          lVar9 = (longlong)puVar2 * 0x1000000;
        }
        FUN_ram_00031e70(&local_160,local_1d0,0,lVar9);
        FUN_ram_00031e70(&local_150,lVar9,0,local_1c0,0);
        FUN_ram_00031e70(&local_130,uVar19,(longlong)uVar19 >> 0x3f,local_1c0,uVar15);
        FUN_ram_00031e70(&local_140,uVar19,(longlong)uVar19 >> 0x3f,local_1d0,0);
        uVar21 = local_130 + local_160 + local_148;
        lVar22 = local_128 + (uVar15 & uVar19) + (ulonglong)(uVar21 < local_130);
        uVar16 = local_158 + ((ulonglong)local_1d0 & lVar9 >> 0x3f & 0x7fffffffffffffff) +
                 (ulonglong)(local_160 + local_148 < local_160);
        uVar19 = uVar16 + local_140;
        uVar15 = uVar19 + lVar22;
        lVar9 = ((longlong)uVar16 >> 0x3f) + local_138 + (ulonglong)(uVar19 < uVar16) +
                (lVar22 >> 0x3f) + (ulonglong)(uVar15 < uVar19);
        uVar19 = (longlong)(uVar15 * 0x10000) >> 0x3f;
        if ((uVar15 >> 0x30 | lVar9 * 0x10000) != uVar19 || lVar9 >> 0x30 != uVar19) {
LAB_ram_0001b3b0:
                    /* WARNING: Subroutine does not return */
          FUN_ram_0002fb80(&DAT_ram_00034280);
        }
        local_1d0 = (undefined *)(uVar15 * 0x10000 | uVar21 >> 0x30);
      }
      else {
        lVar9 = (longlong)uVar19 >> 0x3f;
        FUN_ram_00031e70(&local_80,uVar19,lVar9,puVar17,uVar16);
        FUN_ram_00031e70(&local_70,puVar17,0,puVar17,0);
        FUN_ram_00031e70(&local_60,uVar19,lVar9,uVar19,lVar9);
        uVar21 = local_80 + local_68;
        local_78 = local_78 + (uVar16 & uVar19);
        uVar19 = uVar21 + local_80;
        lVar9 = local_78 + (ulonglong)(uVar19 < uVar21);
        uVar21 = local_78 + (ulonglong)(uVar21 < local_80);
        uVar12 = uVar21 + local_60;
        uVar16 = uVar12 + lVar9;
        lVar9 = ((longlong)uVar21 >> 0x3f) + local_58 + (ulonglong)(uVar12 < uVar21) +
                (lVar9 >> 0x3f) + (ulonglong)(uVar16 < uVar12);
        uVar21 = (longlong)(uVar16 * 0x10000) >> 0x3f;
        if ((uVar16 >> 0x30 | lVar9 * 0x10000) != uVar21 || lVar9 >> 0x30 != uVar21)
        goto LAB_ram_0001b398;
        local_200 = uVar16 * 0x10000 | uVar19 >> 0x30;
        local_1d8 = (undefined *)(uVar19 * 0x10000 | local_70 >> 0x30);
LAB_ram_00019ce8:
        lVar9 = (longlong)local_1d0 >> 0x3f;
        FUN_ram_00031e70(&local_110,local_1d0,lVar9,local_1c0,uVar15);
        FUN_ram_00031e70(&local_100,local_1c0,0,local_1c0,0);
        FUN_ram_00031e70(&local_f0,local_1d0,lVar9,local_1d0,lVar9);
        uVar19 = local_110 + local_f8;
        local_108 = local_108 + (uVar15 & (ulonglong)local_1d0);
        uVar21 = uVar19 + local_110;
        lVar9 = local_108 + (ulonglong)(uVar21 < uVar19);
        uVar16 = local_108 + (ulonglong)(uVar19 < local_110);
        uVar19 = uVar16 + local_f0;
        uVar15 = uVar19 + lVar9;
        lVar9 = ((longlong)uVar16 >> 0x3f) + local_e8 + (ulonglong)(uVar19 < uVar16) +
                (lVar9 >> 0x3f) + (ulonglong)(uVar15 < uVar19);
        uVar19 = (longlong)(uVar15 * 0x10000) >> 0x3f;
        if ((uVar15 >> 0x30 | lVar9 * 0x10000) != uVar19 || lVar9 >> 0x30 != uVar19) {
LAB_ram_0001b398:
                    /* WARNING: Subroutine does not return */
          FUN_ram_0002fb80(&DAT_ram_00034298);
        }
        local_1d0 = (undefined *)(uVar15 * 0x10000 | uVar21 >> 0x30);
        local_150 = local_100;
      }
      local_1c0 = (undefined *)(uVar21 * 0x10000 | local_150 >> 0x30);
      puVar17 = local_1d8;
    }
LAB_ram_0001ac38:
    puVar13 = local_1d0 + (-(ulonglong)(local_1c0 < puVar17) - local_200);
    uVar6 = 0xf6;
    if ((-1 < (longlong)
              (((ulonglong)local_1d0 ^ local_200) & ((ulonglong)local_1d0 ^ (ulonglong)puVar13))) &&
       (uVar6 = 0xf9, puVar20 != (undefined *)0x0 || puVar10 != (undefined *)0x0)) {
      local_20 = (longlong)local_1c0 - (longlong)puVar17;
      lVar9 = -local_20;
      if ((longlong)puVar13 < 0) {
        bVar5 = local_20 != 0;
        local_20 = lVar9;
        if (bVar5) goto LAB_ram_0001acf8;
LAB_ram_0001adf8:
        lVar9 = 0;
        if ((longlong)puVar13 < 0) goto LAB_ram_0001ae10;
LAB_ram_0001ad08:
        local_18 = puVar13;
        if (puVar20 != (undefined *)0x0) goto LAB_ram_0001ad20;
LAB_ram_0001ae38:
        lVar9 = 0;
      }
      else {
        if (local_20 == 0) goto LAB_ram_0001adf8;
LAB_ram_0001acf8:
        lVar9 = 1;
        if (-1 < (longlong)puVar13) goto LAB_ram_0001ad08;
LAB_ram_0001ae10:
        local_18 = (undefined *)-(longlong)(puVar13 + lVar9);
        if (puVar20 == (undefined *)0x0) goto LAB_ram_0001ae38;
LAB_ram_0001ad20:
        lVar9 = 1;
      }
      local_8 = puVar10;
      local_10 = puVar20;
      if ((longlong)puVar10 < 0) {
        local_10 = (undefined *)-(longlong)puVar20;
        local_8 = (undefined *)-(longlong)(puVar10 + lVar9);
      }
      FUN_ram_00001298(&local_50,&local_20,&local_10,0x30);
      if ((char)local_40 == '\0') {
        if ((longlong)((ulonglong)puVar13 ^ (ulonglong)puVar10) < 0) {
          bVar8 = true;
          bVar5 = true;
          if (local_50 == (undefined *)0x0) {
            bVar5 = false;
            if (local_48 < (undefined *)0x8000000000000001) goto LAB_ram_0001af20;
LAB_ram_0001aee0:
            if (local_48 == (undefined *)0x8000000000000000) goto LAB_ram_0001aee8;
LAB_ram_0001af30:
            if (bVar8) goto LAB_ram_0001adb8;
          }
          else {
            if ((undefined *)0x8000000000000000 < local_48) goto LAB_ram_0001aee0;
LAB_ram_0001af20:
            bVar8 = false;
            if (local_48 != (undefined *)0x8000000000000000) goto LAB_ram_0001af30;
LAB_ram_0001aee8:
            if (bVar5) goto LAB_ram_0001adb8;
          }
          puVar17 = (undefined *)-(longlong)local_50;
          puVar20 = (undefined *)((ulonglong)local_48 ^ 0xffffffffffffffff);
          if (puVar17 == (undefined *)0x0) {
            puVar20 = (undefined *)-(longlong)local_48;
          }
        }
        else {
          puVar20 = local_48;
          puVar17 = local_50;
          if ((longlong)local_48 < 0) goto LAB_ram_0001adb8;
        }
        uVar15 = plVar11[1];
        FUN_ram_00031e70(&local_1b0,uVar15,(longlong)uVar15 >> 0x3f,puVar17);
        lVar9 = *plVar11;
        FUN_ram_00031e70(&local_1a0,lVar9,0,puVar17,0);
        FUN_ram_00031e70(&local_180,puVar20,(longlong)puVar20 >> 0x3f,lVar9,lVar9 >> 0x3f);
        FUN_ram_00031e70(&local_190,uVar15,(longlong)uVar15 >> 0x3f,puVar20,
                         (longlong)puVar20 >> 0x3f);
        uVar19 = local_180 + local_1b0 + local_198;
        lVar9 = local_178 + (lVar9 >> 0x3f & (ulonglong)puVar20) + (ulonglong)(uVar19 < local_180);
        uVar16 = local_1a8 + ((longlong)puVar17 >> 0x3f & uVar15) +
                 (ulonglong)(local_1b0 + local_198 < local_1b0);
        uVar21 = uVar16 + local_190;
        uVar15 = uVar21 + lVar9;
        lVar9 = ((longlong)uVar16 >> 0x3f) + local_188 + (ulonglong)(uVar21 < uVar16) +
                (lVar9 >> 0x3f) + (ulonglong)(uVar15 < uVar21);
        uVar16 = (longlong)(uVar15 * 0x10000) >> 0x3f;
        if ((uVar15 >> 0x30 | lVar9 * 0x10000) == uVar16 && lVar9 >> 0x30 == uVar16) {
          *(ulonglong *)(param_1 + 2) = uVar19 * 0x10000 | local_1a0 >> 0x30;
          *(ulonglong *)(param_1 + 4) = uVar15 * 0x10000 | uVar19 >> 0x30;
          uVar6 = 0;
          goto LAB_ram_00018470;
        }
        uVar7 = 0xfd00000000;
        goto LAB_ram_00018460;
      }
    }
LAB_ram_0001adb8:
    param_1[2] = uVar6;
    param_1[1] = 0;
  }
  else {
    uVar7 = 0x8d00000000;
LAB_ram_00018460:
    *(undefined8 *)(param_1 + 1) = uVar7;
  }
  uVar6 = 1;
LAB_ram_00018470:
  *param_1 = uVar6;
  return;
}

// Function: FUN_ram_0001b3c8
void FUN_ram_0001b3c8(undefined4 *param_1,ulonglong param_2,undefined8 param_3,undefined8 param_4,
                     longlong *param_5)

{
  byte bVar1;
  bool bVar2;
  bool bVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  ulonglong uVar8;
  longlong lVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  longlong lVar14;
  ulonglong uVar15;
  longlong lVar16;
  longlong local_180;
  ulonglong local_170;
  ulonglong local_168;
  ulonglong local_160;
  longlong local_158;
  ulonglong local_150;
  longlong local_148;
  longlong local_140;
  longlong local_138;
  ulonglong local_130;
  longlong local_128;
  ulonglong local_120;
  ulonglong local_118;
  ulonglong local_110;
  longlong local_108;
  ulonglong local_100;
  longlong local_f8;
  longlong local_f0;
  longlong local_e8;
  ulonglong local_e0;
  longlong local_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  ulonglong local_c0;
  longlong local_b8;
  ulonglong local_b0;
  longlong local_a8;
  longlong local_a0;
  longlong local_98;
  ulonglong local_90;
  longlong local_88;
  ulonglong local_80;
  longlong local_78;
  ulonglong local_70;
  longlong local_68;
  longlong local_60;
  longlong local_58;
  ulonglong local_50;
  longlong local_48;
  longlong local_40;
  ulonglong local_38;
  undefined *local_30;
  ulonglong local_28;
  ulonglong local_20;
  undefined8 local_18;
  undefined8 local_10;
  
  local_40 = param_2 << 0x30;
  local_38 = ((longlong)param_2 >> 0x3f) << 0x30 | param_2 >> 0x10;
  FUN_ram_00001708(&local_30,&local_40);
  if (local_30 == (undefined *)0x0) {
    uVar7 = 0x8d00000000;
  }
  else {
    local_170 = (local_20 - param_5[3]) - (ulonglong)(local_28 < (ulonglong)param_5[2]);
    if ((longlong)((local_20 ^ param_5[3]) & (local_20 ^ local_170)) < 0) {
      uVar7 = 0x10d00000000;
    }
    else if ((longlong)local_170 < 0) {
      uVar7 = 0x11100000000;
    }
    else {
      bVar1 = *(byte *)(param_5 + 8);
      if (3 < bVar1) {
        local_30 = &DAT_ram_00034258;
        local_10 = 0;
        local_28 = 1;
        local_18 = 0;
        local_20 = 8;
                    /* WARNING: Subroutine does not return */
        FUN_ram_0002fba8(&local_30,&DAT_ram_00034268);
      }
      local_168 = local_28 - param_5[2];
      uVar8 = param_5[1];
      lVar14 = *param_5;
      uVar13 = lVar14 >> 0x3f;
      uVar11 = (longlong)local_168 >> 0x3f;
      lVar9 = (longlong)uVar8 >> 0x3f;
      if (bVar1 < 2) {
        if (bVar1 == 0) {
          if (local_168 == 0 && local_170 == 0) {
            lVar16 = 0;
            uVar11 = 0;
          }
          else {
            if (local_170 == 0) {
              uVar11 = local_168 | local_168 >> 1;
              uVar11 = uVar11 | uVar11 >> 2;
              uVar11 = uVar11 | uVar11 >> 4;
              uVar11 = uVar11 | uVar11 >> 8;
              uVar11 = uVar11 | uVar11 >> 0x10;
              uVar11 = (uVar11 | uVar11 >> 0x20) ^ 0xffffffffffffffff;
              uVar11 = uVar11 - (uVar11 >> 1 & 0x5555555555555555);
              uVar11 = (uVar11 & 0x3333333333333333) + (uVar11 >> 2 & 0x3333333333333333);
              uVar11 = ((uVar11 + (uVar11 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38) +
                       0x40;
            }
            else {
              uVar11 = local_170 | local_170 >> 1;
              uVar11 = uVar11 | uVar11 >> 2;
              uVar11 = uVar11 | uVar11 >> 4;
              uVar11 = uVar11 | uVar11 >> 8;
              uVar11 = uVar11 | uVar11 >> 0x10;
              uVar11 = (uVar11 | uVar11 >> 0x20) ^ 0xffffffffffffffff;
              uVar11 = uVar11 - (uVar11 >> 1 & 0x5555555555555555);
              uVar11 = (uVar11 & 0x3333333333333333) + (uVar11 >> 2 & 0x3333333333333333);
              uVar11 = (uVar11 + (uVar11 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38;
            }
            uVar12 = 0;
            FUN_ram_00031e28(&local_120,1,0,(uVar11 ^ 0xffffffffffffffff) & 0x7e);
            uVar11 = 0;
            do {
              uVar4 = local_120 + uVar12;
              uVar10 = local_118 + uVar11 + (ulonglong)(uVar4 < local_120);
              bVar3 = true;
              if (local_170 < uVar10) {
                if (local_168 < uVar4) goto LAB_ram_0001c498;
LAB_ram_0001c5e0:
                bVar2 = false;
                if (local_170 != uVar10) goto LAB_ram_0001c5f0;
LAB_ram_0001c4a0:
                if (bVar2) goto LAB_ram_0001c4b0;
LAB_ram_0001c608:
                uVar15 = local_118;
                if (!bVar2) goto LAB_ram_0001c620;
LAB_ram_0001c4c0:
                uVar10 = 0;
                if (bVar2) {
LAB_ram_0001c4d0:
                  uVar4 = 0;
                }
              }
              else {
                bVar3 = false;
                if (uVar4 <= local_168) goto LAB_ram_0001c5e0;
LAB_ram_0001c498:
                bVar2 = true;
                if (local_170 == uVar10) goto LAB_ram_0001c4a0;
LAB_ram_0001c5f0:
                bVar2 = bVar3;
                if (!bVar2) goto LAB_ram_0001c608;
LAB_ram_0001c4b0:
                uVar15 = 0;
                if (bVar2) goto LAB_ram_0001c4c0;
LAB_ram_0001c620:
                if (bVar2) goto LAB_ram_0001c4d0;
              }
              uVar5 = 0;
              if (!bVar2) {
                uVar5 = local_120;
              }
              uVar12 = uVar5 + (uVar12 >> 1 | uVar11 << 0x3f);
              if (uVar12 < uVar5) {
                local_180 = 1;
                if (local_168 < uVar4) goto LAB_ram_0001c550;
LAB_ram_0001c688:
                lVar16 = 0;
                if (local_120 < 4) goto LAB_ram_0001c6a0;
LAB_ram_0001c560:
                bVar3 = true;
              }
              else {
                local_180 = 0;
                if (uVar4 <= local_168) goto LAB_ram_0001c688;
LAB_ram_0001c550:
                lVar16 = 1;
                if (3 < local_120) goto LAB_ram_0001c560;
LAB_ram_0001c6a0:
                bVar3 = false;
              }
              if (local_118 != 0) {
                bVar3 = local_118 != 0;
              }
              local_170 = (local_170 - uVar10) - lVar16;
              uVar11 = uVar15 + (uVar11 >> 1) + local_180;
              local_168 = local_168 - uVar4;
              local_120 = local_120 >> 2 | local_118 << 0x3e;
              local_118 = local_118 >> 2;
            } while (bVar3);
            uVar11 = uVar11 * 0x1000000 | uVar12 >> 0x28;
            lVar16 = uVar12 * 0x1000000;
          }
          FUN_ram_00031e70(&local_160,uVar8,lVar9,lVar16);
          FUN_ram_00031e70(&local_150,lVar16,0,lVar14,0);
          FUN_ram_00031e70(&local_130,uVar11,(longlong)uVar11 >> 0x3f,lVar14,uVar13);
          FUN_ram_00031e70(&local_140,uVar11,(longlong)uVar11 >> 0x3f,uVar8,lVar9);
          uVar12 = local_130 + local_160 + local_148;
          lVar9 = local_128 + (uVar13 & uVar11) + (ulonglong)(uVar12 < local_130);
          uVar11 = local_158 + (lVar16 >> 0x3f & uVar8) +
                   (ulonglong)(local_160 + local_148 < local_160);
          uVar13 = uVar11 + local_140;
          uVar8 = uVar13 + lVar9;
          lVar9 = ((longlong)uVar11 >> 0x3f) + local_138 + (ulonglong)(uVar13 < uVar11) +
                  (lVar9 >> 0x3f) + (ulonglong)(uVar8 < uVar13);
          uVar11 = (longlong)(uVar8 * 0x10000) >> 0x3f;
          if ((uVar8 >> 0x30 | lVar9 * 0x10000) == uVar11 && lVar9 >> 0x30 == uVar11) {
            local_20 = uVar8 * 0x10000 | uVar12 >> 0x30;
LAB_ram_0001ca00:
            local_28 = param_5[4] + (uVar12 * 0x10000 | local_150 >> 0x30);
            bVar3 = local_28 < (ulonglong)param_5[4];
            lVar9 = param_5[5];
LAB_ram_0001ca50:
            uVar6 = 0;
            *(ulonglong *)(param_1 + 2) = local_28;
            *(ulonglong *)(param_1 + 4) = lVar9 + local_20 + (ulonglong)bVar3;
            goto LAB_ram_0001b510;
          }
          uVar7 = 0x11800000000;
        }
        else {
          FUN_ram_00031e70(&local_110,uVar8,lVar9,local_168);
          FUN_ram_00031e70(&local_100,lVar14,0,local_168,0);
          FUN_ram_00031e70(&local_e0,local_170,0,lVar14,uVar13);
          FUN_ram_00031e70(&local_f0,uVar8,lVar9,local_170,0);
          uVar12 = local_e0 + local_110 + local_f8;
          lVar9 = local_d8 + (local_170 & uVar13 & 0x7fffffffffffffff) +
                  (ulonglong)(uVar12 < local_e0);
          uVar11 = local_108 + (uVar11 & uVar8) + (ulonglong)(local_110 + local_f8 < local_110);
          uVar13 = uVar11 + local_f0;
          uVar8 = uVar13 + lVar9;
          lVar9 = ((longlong)uVar11 >> 0x3f) + local_e8 + (ulonglong)(uVar13 < uVar11) +
                  (lVar9 >> 0x3f) + (ulonglong)(uVar8 < uVar13);
          uVar11 = (longlong)(uVar8 * 0x10000) >> 0x3f;
          if ((uVar8 >> 0x30 | lVar9 * 0x10000) == uVar11 && lVar9 >> 0x30 == uVar11) {
            local_20 = uVar8 * 0x10000 | uVar12 >> 0x30;
            local_150 = local_100;
            goto LAB_ram_0001ca00;
          }
          uVar7 = 0x11d00000000;
        }
      }
      else if (bVar1 == 2) {
        FUN_ram_00031e70(&local_c0,uVar8,lVar9,local_168);
        FUN_ram_00031e70(&local_b0,lVar14,0,local_168,0);
        FUN_ram_00031e70(&local_90,local_170,0,lVar14,uVar13);
        FUN_ram_00031e70(&local_a0,uVar8,lVar9,local_170,0);
        uVar12 = local_90 + local_c0 + local_a8;
        lVar9 = local_88 + (local_170 & uVar13 & 0x7fffffffffffffff) +
                (ulonglong)(uVar12 < local_90);
        uVar8 = local_b8 + (uVar11 & uVar8) + (ulonglong)(local_c0 + local_a8 < local_c0);
        uVar11 = uVar8 + local_a0;
        uVar13 = uVar11 + lVar9;
        lVar9 = ((longlong)uVar8 >> 0x3f) + local_98 + (ulonglong)(uVar11 < uVar8) + (lVar9 >> 0x3f)
                + (ulonglong)(uVar13 < uVar11);
        uVar8 = (longlong)(uVar13 * 0x10000) >> 0x3f;
        if ((uVar13 >> 0x30 | lVar9 * 0x10000) == uVar8 && lVar9 >> 0x30 == uVar8) {
          FUN_ram_000009a8(&local_d0,local_168,local_170);
          FUN_ram_000005a8(&local_30,uVar12 * 0x10000 | local_b0 >> 0x30,
                           uVar13 * 0x10000 | uVar12 >> 0x30,local_d0,local_c8);
          if (local_30 != (undefined *)0x0) {
LAB_ram_0001c178:
            local_28 = param_5[4] + local_28;
            bVar3 = local_28 < (ulonglong)param_5[4];
            lVar9 = param_5[5];
            goto LAB_ram_0001ca50;
          }
          uVar7 = 0x12700000000;
        }
        else {
          uVar7 = 0x12300000000;
        }
      }
      else {
        FUN_ram_00031e70(&local_80,uVar8,lVar9,local_168);
        FUN_ram_00031e70(&local_70,lVar14,0,local_168,0);
        FUN_ram_00031e70(&local_50,local_170,0,lVar14,uVar13);
        FUN_ram_00031e70(&local_60,uVar8,lVar9,local_170,0);
        uVar12 = local_50 + local_80 + local_68;
        lVar9 = local_48 + (local_170 & uVar13 & 0x7fffffffffffffff) +
                (ulonglong)(uVar12 < local_50);
        uVar8 = local_78 + (uVar11 & uVar8) + (ulonglong)(local_80 + local_68 < local_80);
        uVar13 = uVar8 + local_60;
        uVar11 = uVar13 + lVar9;
        lVar9 = ((longlong)uVar8 >> 0x3f) + local_58 + (ulonglong)(uVar13 < uVar8) + (lVar9 >> 0x3f)
                + (ulonglong)(uVar11 < uVar13);
        uVar8 = (longlong)(uVar11 * 0x10000) >> 0x3f;
        if ((uVar11 >> 0x30 | lVar9 * 0x10000) == uVar8 && lVar9 >> 0x30 == uVar8) {
          FUN_ram_000005a8(&local_30,uVar12 * 0x10000 | local_70 >> 0x30,
                           uVar11 * 0x10000 | uVar12 >> 0x30,local_168,local_170);
          if (local_30 != (undefined *)0x0) goto LAB_ram_0001c178;
          uVar7 = 0x13100000000;
        }
        else {
          uVar7 = 0x12e00000000;
        }
      }
    }
  }
  *(undefined8 *)(param_1 + 1) = uVar7;
  uVar6 = 1;
LAB_ram_0001b510:
  *param_1 = uVar6;
  return;
}

// Function: FUN_ram_0001cb08
/* WARNING: Type propagation algorithm not settling */

void FUN_ram_0001cb08(undefined4 *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                     longlong param_5)

{
  bool bVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  undefined1 uVar4;
  bool bVar5;
  ulonglong uVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  longlong lVar12;
  longlong lVar13;
  byte bVar14;
  undefined4 uVar15;
  undefined8 uVar16;
  byte bVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  undefined8 uVar20;
  ulonglong uVar21;
  ulonglong uVar22;
  ulonglong uVar23;
  longlong lVar24;
  longlong lVar25;
  undefined8 uVar26;
  longlong local_40;
  ulonglong local_38;
  undefined8 local_28;
  undefined8 local_20;
  int local_18;
  uint uStack_14;
  undefined4 local_10;
  undefined4 uStack_c;
  longlong local_8;
  
  local_28 = param_2;
  local_20 = param_3;
  FUN_ram_00023e58(&local_18,&local_28);
  lVar24 = local_8;
  if (local_18 == 0) {
    lVar25 = *(longlong *)(param_5 + -0xfe8);
    uVar16 = *(undefined8 *)(param_5 + -0xff0);
    uVar26 = *(undefined8 *)(param_5 + -0xff8);
    uVar20 = *(undefined8 *)(param_5 + -0x1000);
    uVar6 = CONCAT44(uStack_c,local_10);
    FUN_ram_00024af8(&local_18,&local_28);
    lVar7 = local_8;
    if (local_18 == 0) {
      local_38 = CONCAT44(uStack_c,local_10);
      FUN_ram_0001de10(&local_18,&local_28,param_4);
      lVar8 = local_8;
      if (local_18 == 0) {
        uVar19 = CONCAT44(uStack_c,local_10);
        FUN_ram_0001fe70(&local_18,&local_28,param_4);
        lVar9 = local_8;
        if (local_18 == 0) {
          uVar2 = CONCAT44(uStack_c,local_10);
          FUN_ram_000216f0(&local_18,&local_28,param_4,uVar20,uVar26);
          lVar10 = local_8;
          if (local_18 == 0) {
            uVar3 = CONCAT44(uStack_c,local_10);
            FUN_ram_00022c08(&local_18,&local_28,param_4);
            lVar11 = local_8;
            if (local_18 == 0) {
              uVar21 = CONCAT44(uStack_c,local_10);
              FUN_ram_0001fbf0(&local_18,&local_28);
              lVar12 = local_8;
              if (local_18 == 0) {
                uVar22 = CONCAT44(uStack_c,local_10);
                FUN_ram_000266a0(&local_18,&local_28,uVar16);
                lVar13 = local_8;
                if (local_18 == 0) {
                  uVar23 = CONCAT44(uStack_c,local_10);
                  if (lVar25 == 0) {
                    FUN_ram_00000908(&local_18,0xc8000000000000,0,0x3e8000000000000,0);
                    local_40 = CONCAT44(uStack_c,local_10);
                    uVar18 = CONCAT44(uStack_14,local_18);
                  }
                  else {
                    uVar18 = 0;
                    local_40 = 0;
                  }
                  bVar1 = true;
                  if (local_38 < uVar6) {
                    if (lVar7 < lVar24) goto LAB_ram_0001cf60;
LAB_ram_0001d250:
                    bVar5 = false;
                    if (lVar24 != lVar7) goto LAB_ram_0001d260;
LAB_ram_0001cf68:
                    if (bVar1) goto LAB_ram_0001cf70;
LAB_ram_0001d270:
                    if (!bVar1) goto LAB_ram_0001d280;
LAB_ram_0001cf78:
                    lVar7 = lVar24;
                    if (lVar8 < lVar24) goto LAB_ram_0001cf88;
LAB_ram_0001d298:
                    bVar1 = false;
                    if (local_38 <= uVar19) goto LAB_ram_0001d2c0;
LAB_ram_0001cfa8:
                    bVar5 = true;
                    if (lVar7 == lVar8) goto LAB_ram_0001cfb0;
LAB_ram_0001d2d0:
                    bVar5 = bVar1;
                    if (!bVar5) goto LAB_ram_0001d2e0;
LAB_ram_0001cfb8:
                    if (bVar5) goto LAB_ram_0001cfc0;
LAB_ram_0001d2f8:
                    if (lVar8 <= lVar9) goto LAB_ram_0001d318;
LAB_ram_0001cfd8:
                    bVar1 = true;
                    if (uVar2 < local_38) goto LAB_ram_0001cff8;
LAB_ram_0001d340:
                    bVar5 = false;
                    if (lVar8 != lVar9) goto LAB_ram_0001d358;
LAB_ram_0001d008:
                    if (bVar5) goto LAB_ram_0001d018;
LAB_ram_0001d370:
                    local_38 = uVar2;
                    if (!bVar5) goto LAB_ram_0001d388;
LAB_ram_0001d020:
                    lVar9 = lVar8;
                    if (lVar10 < lVar9) goto LAB_ram_0001d030;
LAB_ram_0001d3a0:
                    bVar1 = false;
                    if (local_38 <= uVar3) goto LAB_ram_0001d3c0;
LAB_ram_0001d048:
                    bVar5 = true;
                    if (lVar9 == lVar10) goto LAB_ram_0001d050;
LAB_ram_0001d3d0:
                    bVar5 = bVar1;
                    if (!bVar5) goto LAB_ram_0001d3f8;
LAB_ram_0001d070:
                    if (bVar5) goto LAB_ram_0001d080;
LAB_ram_0001d410:
                    if (lVar10 <= lVar11) goto LAB_ram_0001d430;
LAB_ram_0001d098:
                    bVar1 = true;
                    if (uVar21 < local_38) goto LAB_ram_0001d0b0;
LAB_ram_0001d450:
                    bVar5 = false;
                    if (lVar10 != lVar11) goto LAB_ram_0001d460;
LAB_ram_0001d0b8:
                    if (bVar5) goto LAB_ram_0001d0c0;
LAB_ram_0001d470:
                    if (!bVar5) goto LAB_ram_0001d480;
LAB_ram_0001d0c8:
                    lVar11 = lVar10;
                    if (lVar12 < lVar11) goto LAB_ram_0001d0e0;
LAB_ram_0001d4a0:
                    bVar1 = false;
                    if (uVar21 <= uVar22) goto LAB_ram_0001d4c8;
LAB_ram_0001d100:
                    bVar5 = true;
                    if (lVar11 == lVar12) goto LAB_ram_0001d110;
LAB_ram_0001d4e0:
                    bVar5 = bVar1;
                    if (!bVar5) goto LAB_ram_0001d4f0;
LAB_ram_0001d118:
                    uVar22 = uVar21;
                    if (bVar5) goto LAB_ram_0001d128;
LAB_ram_0001d508:
                    if (lVar12 <= lVar13) goto LAB_ram_0001d528;
LAB_ram_0001d140:
                    bVar1 = true;
                    if (uVar23 < uVar22) goto LAB_ram_0001d150;
LAB_ram_0001d540:
                    bVar5 = false;
                    if (lVar12 != lVar13) goto LAB_ram_0001d550;
LAB_ram_0001d158:
                    if (bVar5) goto LAB_ram_0001d160;
LAB_ram_0001d560:
                    if (!bVar5) goto LAB_ram_0001d570;
LAB_ram_0001d168:
                    lVar13 = lVar12;
                    if (local_40 < lVar13) goto LAB_ram_0001d178;
LAB_ram_0001d588:
                    bVar1 = false;
                    if (uVar23 <= uVar18) goto LAB_ram_0001d5a0;
LAB_ram_0001d188:
                    bVar5 = true;
                    if (lVar13 == local_40) goto LAB_ram_0001d190;
LAB_ram_0001d5b0:
                    bVar5 = bVar1;
                    if (!bVar5) goto LAB_ram_0001d5c0;
LAB_ram_0001d198:
                    local_40 = lVar13;
                    if (bVar5) goto LAB_ram_0001d1a0;
LAB_ram_0001d5d0:
                    if (uVar18 < 0x1000000000000) goto LAB_ram_0001d5f8;
LAB_ram_0001d1c0:
                    bVar1 = true;
                    if (0 < local_40) goto LAB_ram_0001d1d0;
LAB_ram_0001d610:
                    bVar5 = false;
                    if (local_40 != 0) goto LAB_ram_0001d620;
LAB_ram_0001d1d8:
                    bVar5 = bVar1;
                    if ((uVar18 & 0xffffffffffff) != 0) goto LAB_ram_0001d1f8;
LAB_ram_0001d648:
                    bVar17 = 0;
                    if (0xffffffffffff < uVar18) goto LAB_ram_0001d668;
LAB_ram_0001d210:
                    bVar14 = 1;
                    if (local_40 == 0) goto LAB_ram_0001d218;
LAB_ram_0001d678:
                    bVar14 = 0;
                    if (bVar5) goto LAB_ram_0001d778;
LAB_ram_0001d698:
                    if ((bool)(bVar14 & bVar17)) goto LAB_ram_0001d778;
                    uVar6 = FUN_ram_0001dbd0(&local_28,&DAT_ram_000335c0);
                    uVar15 = (undefined4)(uVar6 >> 0x20);
                    if ((uVar6 & 0xffffffff) != 0x1a) goto LAB_ram_0001ce48;
                    if ((uVar6 >> 0x20 & 1) == 0) {
                      uVar6 = FUN_ram_0001dbd0(&local_28,&DAT_ram_00033660);
                      uVar15 = (undefined4)(uVar6 >> 0x20);
                      if ((uVar6 & 0xffffffff) != 0x1a) goto LAB_ram_0001ce48;
                      if ((uVar6 >> 0x20 & 1) != 0) goto LAB_ram_0001d718;
                    }
                    else {
LAB_ram_0001d718:
                      FUN_ram_00000908(&local_18,0x677000000000000,0,0x3e8000000000000,0);
                      local_40 = CONCAT44(uStack_c,local_10);
                      uVar18 = CONCAT44(uStack_14,local_18);
                    }
LAB_ram_0001d8a8:
                    FUN_ram_00025bc8(&local_18,&local_28);
                    lVar24 = local_8;
                    if (local_18 == 0) {
                      uVar6 = CONCAT44(uStack_c,local_10);
                      FUN_ram_00026078(&local_18,&local_28);
                      if (local_18 == 0) {
                        uVar18 = uVar6 + uVar18;
                        lVar24 = lVar24 + local_40 + (ulonglong)(uVar18 < uVar6) + local_8;
                        uVar6 = uVar18 + CONCAT44(uStack_c,local_10);
                        if (uVar6 < uVar18) {
                          lVar24 = lVar24 + 1;
                        }
                        uVar4 = 0xffffffffffff < uVar6;
                        if (lVar24 != 0) {
                          uVar4 = 0 < lVar24;
                        }
                        if (((bool)uVar4) ||
                           ((lVar24 == 0 && uVar6 < 0x1000000000000) &&
                            (uVar6 & 0xffffffffffff) != 0)) {
                          FUN_ram_0001dba8(1,0);
                        }
                        *(ulonglong *)(param_1 + 2) = uVar6;
                        *(longlong *)(param_1 + 4) = lVar24;
                        *param_1 = 0;
                        return;
                      }
                    }
                  }
                  else {
                    bVar1 = false;
                    if (lVar24 <= lVar7) goto LAB_ram_0001d250;
LAB_ram_0001cf60:
                    bVar5 = true;
                    if (lVar24 == lVar7) goto LAB_ram_0001cf68;
LAB_ram_0001d260:
                    bVar1 = bVar5;
                    if (!bVar1) goto LAB_ram_0001d270;
LAB_ram_0001cf70:
                    local_38 = uVar6;
                    if (bVar1) goto LAB_ram_0001cf78;
LAB_ram_0001d280:
                    if (lVar7 <= lVar8) goto LAB_ram_0001d298;
LAB_ram_0001cf88:
                    bVar1 = true;
                    if (uVar19 < local_38) goto LAB_ram_0001cfa8;
LAB_ram_0001d2c0:
                    bVar5 = false;
                    if (lVar7 != lVar8) goto LAB_ram_0001d2d0;
LAB_ram_0001cfb0:
                    if (bVar5) goto LAB_ram_0001cfb8;
LAB_ram_0001d2e0:
                    local_38 = uVar19;
                    if (!bVar5) goto LAB_ram_0001d2f8;
LAB_ram_0001cfc0:
                    lVar8 = lVar7;
                    if (lVar9 < lVar8) goto LAB_ram_0001cfd8;
LAB_ram_0001d318:
                    bVar1 = false;
                    if (local_38 <= uVar2) goto LAB_ram_0001d340;
LAB_ram_0001cff8:
                    bVar5 = true;
                    if (lVar8 == lVar9) goto LAB_ram_0001d008;
LAB_ram_0001d358:
                    bVar5 = bVar1;
                    if (!bVar5) goto LAB_ram_0001d370;
LAB_ram_0001d018:
                    if (bVar5) goto LAB_ram_0001d020;
LAB_ram_0001d388:
                    if (lVar9 <= lVar10) goto LAB_ram_0001d3a0;
LAB_ram_0001d030:
                    bVar1 = true;
                    if (uVar3 < local_38) goto LAB_ram_0001d048;
LAB_ram_0001d3c0:
                    bVar5 = false;
                    if (lVar9 != lVar10) goto LAB_ram_0001d3d0;
LAB_ram_0001d050:
                    if (bVar5) goto LAB_ram_0001d070;
LAB_ram_0001d3f8:
                    local_38 = uVar3;
                    if (!bVar5) goto LAB_ram_0001d410;
LAB_ram_0001d080:
                    lVar10 = lVar9;
                    if (lVar11 < lVar10) goto LAB_ram_0001d098;
LAB_ram_0001d430:
                    bVar1 = false;
                    if (local_38 <= uVar21) goto LAB_ram_0001d450;
LAB_ram_0001d0b0:
                    bVar5 = true;
                    if (lVar10 == lVar11) goto LAB_ram_0001d0b8;
LAB_ram_0001d460:
                    bVar5 = bVar1;
                    if (!bVar5) goto LAB_ram_0001d470;
LAB_ram_0001d0c0:
                    uVar21 = local_38;
                    if (bVar5) goto LAB_ram_0001d0c8;
LAB_ram_0001d480:
                    if (lVar11 <= lVar12) goto LAB_ram_0001d4a0;
LAB_ram_0001d0e0:
                    bVar1 = true;
                    if (uVar22 < uVar21) goto LAB_ram_0001d100;
LAB_ram_0001d4c8:
                    bVar5 = false;
                    if (lVar11 != lVar12) goto LAB_ram_0001d4e0;
LAB_ram_0001d110:
                    if (bVar5) goto LAB_ram_0001d118;
LAB_ram_0001d4f0:
                    if (!bVar5) goto LAB_ram_0001d508;
LAB_ram_0001d128:
                    lVar12 = lVar11;
                    if (lVar13 < lVar12) goto LAB_ram_0001d140;
LAB_ram_0001d528:
                    bVar1 = false;
                    if (uVar22 <= uVar23) goto LAB_ram_0001d540;
LAB_ram_0001d150:
                    bVar5 = true;
                    if (lVar12 == lVar13) goto LAB_ram_0001d158;
LAB_ram_0001d550:
                    bVar5 = bVar1;
                    if (!bVar5) goto LAB_ram_0001d560;
LAB_ram_0001d160:
                    uVar23 = uVar22;
                    if (bVar5) goto LAB_ram_0001d168;
LAB_ram_0001d570:
                    if (lVar13 <= local_40) goto LAB_ram_0001d588;
LAB_ram_0001d178:
                    bVar1 = true;
                    if (uVar18 < uVar23) goto LAB_ram_0001d188;
LAB_ram_0001d5a0:
                    bVar5 = false;
                    if (lVar13 != local_40) goto LAB_ram_0001d5b0;
LAB_ram_0001d190:
                    if (bVar5) goto LAB_ram_0001d198;
LAB_ram_0001d5c0:
                    if (!bVar5) goto LAB_ram_0001d5d0;
LAB_ram_0001d1a0:
                    uVar18 = uVar23;
                    if (0xffffffffffff < uVar18) goto LAB_ram_0001d1c0;
LAB_ram_0001d5f8:
                    bVar1 = false;
                    if (local_40 < 1) goto LAB_ram_0001d610;
LAB_ram_0001d1d0:
                    bVar5 = true;
                    if (local_40 == 0) goto LAB_ram_0001d1d8;
LAB_ram_0001d620:
                    if ((uVar18 & 0xffffffffffff) == 0) goto LAB_ram_0001d648;
LAB_ram_0001d1f8:
                    bVar17 = 1;
                    if (uVar18 < 0x1000000000000) goto LAB_ram_0001d210;
LAB_ram_0001d668:
                    bVar14 = 0;
                    if (local_40 != 0) goto LAB_ram_0001d678;
LAB_ram_0001d218:
                    if (!bVar5) goto LAB_ram_0001d698;
LAB_ram_0001d778:
                    FUN_ram_00025718(&local_18,&local_28);
                    lVar24 = local_8;
                    if (local_18 == 0) {
                      uVar6 = CONCAT44(uStack_c,local_10);
                      FUN_ram_00026238(&local_18,&local_28,param_4);
                      if (local_18 == 0) {
                        uVar19 = uVar6 + uVar18;
                        uVar18 = uVar19 + CONCAT44(uStack_c,local_10);
                        local_40 = lVar24 + local_40 + (ulonglong)(uVar19 < uVar6) + local_8 +
                                   (ulonglong)(uVar18 < uVar19);
                        goto LAB_ram_0001d8a8;
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  uVar6 = (ulonglong)uStack_14;
  uVar15 = local_10;
LAB_ram_0001ce48:
  param_1[2] = uVar15;
  param_1[1] = (int)uVar6;
  *param_1 = 1;
  return;
}

// Function: FUN_ram_0001dba8
void FUN_ram_0001dba8(void)

{
  undefined4 *unaff_R6;
  undefined8 unaff_R7;
  undefined8 unaff_R8;
  
  FUN_ram_0001dba8();
  *(undefined8 *)(unaff_R6 + 2) = unaff_R8;
  *(undefined8 *)(unaff_R6 + 4) = unaff_R7;
  *unaff_R6 = 0;
  return;
}

// Function: FUN_ram_0001dbd0
undefined8 FUN_ram_0001dbd0(undefined8 *param_1,longlong *param_2)

{
  bool bVar1;
  ushort *puVar2;
  ulonglong uVar3;
  ushort *puVar4;
  longlong *plVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  
  puVar2 = (ushort *)*param_1;
  if ((ulonglong)*puVar2 != 0) {
    uVar3 = 0;
    do {
      if (uVar3 != *(ushort *)((longlong)puVar2 + param_1[1] + -2)) {
        puVar4 = (ushort *)((longlong)puVar2 + (ulonglong)puVar2[uVar3 + 1]);
        uVar6 = (ulonglong)*puVar4;
        if ((((*(longlong *)((longlong)puVar4 + uVar6 * 0x21 + 2) != *param_2) ||
             (*(longlong *)((longlong)puVar4 + uVar6 * 0x21 + 10) != param_2[1])) ||
            (*(longlong *)((longlong)puVar4 + uVar6 * 0x21 + 0x12) != param_2[2])) ||
           (bVar1 = false, *(longlong *)((longlong)puVar4 + uVar6 * 0x21 + 0x1a) != param_2[3])) {
          bVar1 = true;
        }
        if (!bVar1) {
          return 0x10000001a;
        }
        if (uVar6 != 0) {
          plVar5 = (longlong *)((longlong)puVar4 + 3);
          uVar7 = 0;
          do {
            if (((*plVar5 != *param_2) || (plVar5[1] != param_2[1])) ||
               ((plVar5[2] != param_2[2] || (bVar1 = false, plVar5[3] != param_2[3])))) {
              bVar1 = true;
            }
            if (!bVar1) {
              return 0x10000001a;
            }
            plVar5 = (longlong *)((longlong)plVar5 + 0x21);
            uVar7 = uVar7 + 1;
          } while (uVar7 < uVar6);
        }
      }
      uVar3 = uVar3 + 1;
    } while (uVar3 < *puVar2);
  }
  return 0x1a;
}

// Function: FUN_ram_0001de10
void FUN_ram_0001de10(undefined4 *param_1,undefined8 *param_2,longlong *param_3)

{
  ushort uVar1;
  bool bVar2;
  ushort *puVar3;
  ulonglong uVar4;
  ushort *puVar5;
  longlong *plVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  undefined8 local_20;
  undefined8 local_18;
  undefined8 local_10;
  undefined8 local_8;
  
  if ((((*param_3 != 0x6560b6dd6ee140db) || (param_3[1] != -0x163a020e81a9d7c4)) ||
      (param_3[2] != 0x10742a9290fc845b)) || (bVar2 = false, param_3[3] != 0x182e105eeb8708ad)) {
    bVar2 = true;
  }
  if (bVar2) {
    if (((*param_3 != -0x724bde0defd224ab) || (param_3[1] != -0x4ddc648a4ff5d960)) ||
       ((param_3[2] != 0x17f86227578d7956 || (bVar2 = false, param_3[3] != -0x2866c6493ba65e0d)))) {
      bVar2 = true;
    }
    if (bVar2) {
      if (((*param_3 != -0x5b792ad20f59a16d) || (param_3[1] != -0x2c15f09912347a81)) ||
         ((param_3[2] != 0xccbecd97e436386 || (bVar2 = false, param_3[3] != 0x3aa82a4cb9d28622)))) {
        bVar2 = true;
      }
      if (bVar2) {
        if ((((*param_3 != 0x6d46af69e74bdfb4) || (param_3[1] != -0xfb7c095e807c65c)) ||
            (param_3[2] != -0x49badeacf352c099)) ||
           (bVar2 = false, param_3[3] != 0x494ac5dec856a9e9)) {
          bVar2 = true;
        }
        if (bVar2) {
          if (((*param_3 != 0x602eddf9a6f50302) || (param_3[1] != -0x30262668f50f63cf)) ||
             ((param_3[2] != 0x5f9eab07c0325e41 || (bVar2 = false, param_3[3] != 0x74308a941848db62)
              ))) {
            bVar2 = true;
          }
          if (bVar2) {
            if (((*param_3 != -0x5a8406c1ee9740dd) || (param_3[1] != 0x694d916b33d303ed)) ||
               ((param_3[2] != -0x5a746ce81742b801 ||
                (bVar2 = false, param_3[3] != 0x77eb01650c19b51)))) {
              bVar2 = true;
            }
            local_20 = 0;
            local_18 = 0;
            if (bVar2) goto LAB_ram_0001ef00;
          }
        }
      }
    }
  }
  local_20 = 0;
  puVar3 = (ushort *)*param_2;
  uVar7 = (ulonglong)*(ushort *)((longlong)puVar3 + param_2[1] + -2);
  uVar4 = (ulonglong)*puVar3;
  local_18 = 0;
  if (uVar7 < uVar4) {
    puVar5 = (ushort *)((longlong)puVar3 + (ulonglong)puVar3[uVar7 + 1]);
    uVar1 = *puVar5;
    plVar6 = (longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 2);
    if ((((*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 2) != 0x4873bce2144ae3b5) ||
         (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) != -0x2911a2500a1ef197)) ||
        (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x60b8aa6da3403855)) ||
       (bVar2 = false,
       *(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) != 0x103cc0bd736050b0)) {
      bVar2 = true;
    }
    if (bVar2) {
      if (((*plVar6 != -0x1e8395f2e7b51c4b) ||
          (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) != -0x51f325fec501496b))
         || ((*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x98144e7e5ae3fa8
             || (bVar2 = false,
                *(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                0x40ee2497930cf7ea)))) {
        bVar2 = true;
      }
      if (bVar2) {
        if (((*plVar6 != 0x6ec031f25bd57904) ||
            (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) != 0x71568ce6ec574ee))
           || ((*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                0x518ef4a3deb2b1fd ||
               (bVar2 = false,
               *(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
               -0x70ec43a95d324efe)))) {
          bVar2 = true;
        }
        if (bVar2) {
          if ((((*plVar6 != 0x715b8f7af9be1205) ||
               (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) !=
                -0x3fbd123929120c83)) ||
              (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
               -0x1178411a20edb01e)) ||
             (bVar2 = false,
             *(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) != -0x4693a2c08ba113c1
             )) {
            bVar2 = true;
          }
          if (bVar2) {
            if (((*plVar6 != -0x3b66289859b23cf6) ||
                (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) !=
                 0x75b1926ae1365115)) ||
               ((*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                 0x678ad2090231d088 ||
                (bVar2 = false,
                *(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                -0x139993aed94b961d)))) {
              bVar2 = true;
            }
            if (bVar2) {
              if (((*plVar6 != 0x136d5ca2f1569155) ||
                  (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) !=
                   0x340d9a0ae6f72a4f)) ||
                 ((*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                   -0x2a9d9b9ca96e3882 ||
                  (bVar2 = false,
                  *(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                  0x698f3435f126add1)))) {
                bVar2 = true;
              }
              if (bVar2) {
                if ((((*plVar6 != -0x16a608d8d48b0286) ||
                     (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) !=
                      0x7a819dd33c7070c6)) ||
                    (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                     0x6dd2523bce0a93a0)) ||
                   (bVar2 = false,
                   *(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                   -0x2c4478dc22ab5fac)) {
                  bVar2 = true;
                }
                if (bVar2) {
                  if (((*plVar6 != -0x44f118ed916356fa) ||
                      (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) !=
                       0x6e904b4c145c1835)) ||
                     ((*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                       0x2a2f74470ab0ff18 ||
                      (bVar2 = false,
                      *(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                      -0x2b367796f4eefba2)))) {
                    bVar2 = true;
                  }
                  if (bVar2) {
                    if (((*plVar6 != -0x4fc4eec7e6cb4135) ||
                        (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) !=
                         0x45acad558b7e296b)) ||
                       ((*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                         0x59369b4a1734ee6f ||
                        (bVar2 = false,
                        *(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                        0x42c79970523f5e6b)))) {
                      bVar2 = true;
                    }
                    if (bVar2) {
                      if ((((*plVar6 != -0x1d323195ffe246f3) ||
                           (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) !=
                            0x67889bcdcd17de84)) ||
                          (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                           0x5666dfd02b922d2b)) ||
                         (bVar2 = false,
                         *(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                         0x548b03e01a423aa3)) {
                        bVar2 = true;
                      }
                      if (bVar2) {
                        if (((*plVar6 != -0x6c2c22b8abad132c) ||
                            (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) !=
                             0x1776bd19d4d98a5b)) ||
                           ((*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                             0x6f034a62de39afcb ||
                            (bVar2 = false,
                            *(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                            -0x5f19bd0c7dda6fc5)))) {
                          bVar2 = true;
                        }
                        if (bVar2) {
                          if (((*plVar6 != -0x1bb09aaaa3eacf65) ||
                              (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) !=
                               0x6493c705f351bd52)) ||
                             ((*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                               0x262c1d3289763901 ||
                              (bVar2 = false,
                              *(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                              0x5be22f238cb47253)))) {
                            bVar2 = true;
                          }
                          if (bVar2) {
                            if ((((*plVar6 != -0x7af703e2864bdf4) ||
                                 (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) !=
                                  0x2de7dd1cfc9a6d15)) ||
                                (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                 0x6bafec3babd968f6)) ||
                               (bVar2 = false,
                               *(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                               -0x3726a59b99a8f2a9)) {
                              bVar2 = true;
                            }
                            if (bVar2) {
                              if (((*plVar6 != -0x372c55a8b3c334fc) ||
                                  (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) !=
                                   0x72e40dd1add9f2d5)) ||
                                 ((*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12)
                                   != 0x42e6fdaa3eff7804 ||
                                  (bVar2 = false,
                                  *(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a)
                                  != -0x3a991ec56a126c8d)))) {
                                bVar2 = true;
                              }
                              if (bVar2) {
                                if (((*plVar6 != -0xc5e8ffce1a16dfa) ||
                                    (*(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10)
                                     != -0x2070af22c1e0392a)) ||
                                   ((*(longlong *)
                                      ((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                     -0x4d27c110388d62ba ||
                                    (bVar2 = false,
                                    *(longlong *)((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a)
                                    != -0x19ea30d62c1318f6)))) {
                                  bVar2 = true;
                                }
                                if (bVar2) {
                                  if ((((*plVar6 != -0x241f8dfce1a16dfa) ||
                                       (*(longlong *)
                                         ((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) !=
                                        0x77ca68769172b20b)) ||
                                      (*(longlong *)
                                        ((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                       0x533f7524d0ace446)) ||
                                     (bVar2 = false,
                                     *(longlong *)
                                      ((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                     -0x6567b74b076d7538)) {
                                    bVar2 = true;
                                  }
                                  if (bVar2) {
                                    if (((*plVar6 != 0x1be9073efd071895) ||
                                        (*(longlong *)
                                          ((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) !=
                                         0x103eb598830568a5)) ||
                                       ((*(longlong *)
                                          ((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                         -0x6f5cf633300ceda6 ||
                                        (bVar2 = false,
                                        *(longlong *)
                                         ((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                        -0x5ca0aa4a02280026)))) {
                                      bVar2 = true;
                                    }
                                    if (bVar2) {
                                      if (((*plVar6 != -0x2d9d51bf92ab29f4) ||
                                          (*(longlong *)
                                            ((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) !=
                                           0x45eefdfe7495b816)) ||
                                         ((*(longlong *)
                                            ((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                           0xbb5f49c7d946b85 ||
                                          (bVar2 = false,
                                          *(longlong *)
                                           ((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                          0x115c61a060bb5829)))) {
                                        bVar2 = true;
                                      }
                                      if (bVar2) {
                                        if ((((*plVar6 != 0x366a33d8ef74db2b) ||
                                             (*(longlong *)
                                               ((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 10) !=
                                              0x6819eac7d96353c0)) ||
                                            (*(longlong *)
                                              ((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                             0x2a05877358342528)) ||
                                           (bVar2 = false,
                                           *(longlong *)
                                            ((longlong)puVar5 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                           -0x2d7431cce59f6330)) {
                                          bVar2 = true;
                                        }
                                        if (bVar2) {
                                          if (((*param_3 != 0x6560b6dd6ee140db) ||
                                              (param_3[1] != -0x163a020e81a9d7c4)) ||
                                             ((param_3[2] != 0x10742a9290fc845b ||
                                              (bVar2 = false, param_3[3] != 0x182e105eeb8708ad)))) {
                                            bVar2 = true;
                                          }
                                          if (bVar2) {
                                            local_8 = 0x182e105eeb8708ad;
                                            local_10 = 0x10742a9290fc845b;
                                            local_18 = 0xe9c5fdf17e56283c;
                                            local_20 = 0x6560b6dd6ee140db;
                                            uVar7 = 0;
                                            do {
                                              uVar8 = (ulonglong)
                                                      *(ushort *)
                                                       ((longlong)puVar3 +
                                                       (ulonglong)puVar3[uVar7 + 1]);
                                              if (uVar8 != 0) {
                                                plVar6 = (longlong *)
                                                         ((longlong)
                                                          ((longlong)puVar3 +
                                                          (ulonglong)puVar3[uVar7 + 1]) + 3);
                                                uVar9 = 0;
                                                do {
                                                  if (((*plVar6 != 0x6560b6dd6ee140db) ||
                                                      (plVar6[1] != -0x163a020e81a9d7c4)) ||
                                                     ((plVar6[2] != 0x10742a9290fc845b ||
                                                      (bVar2 = false,
                                                      plVar6[3] != 0x182e105eeb8708ad)))) {
                                                    bVar2 = true;
                                                  }
                                                  if (!bVar2) goto LAB_ram_0001fb60;
                                                  plVar6 = (longlong *)((longlong)plVar6 + 0x21);
                                                  uVar9 = uVar9 + 1;
                                                } while (uVar9 < uVar8);
                                              }
                                              uVar7 = uVar7 + 1;
                                            } while (uVar7 < uVar4);
                                          }
                                          if ((((*param_3 != -0x724bde0defd224ab) ||
                                               (param_3[1] != -0x4ddc648a4ff5d960)) ||
                                              (param_3[2] != 0x17f86227578d7956)) ||
                                             (bVar2 = false, param_3[3] != -0x2866c6493ba65e0d)) {
                                            bVar2 = true;
                                          }
                                          if (bVar2) {
                                            local_8 = 0xd79939b6c459a1f3;
                                            local_10 = 0x17f86227578d7956;
                                            local_18 = 0xb2239b75b00a26a0;
                                            local_20 = 0x8db421f2102ddb55;
                                            uVar7 = 0;
                                            do {
                                              uVar8 = (ulonglong)
                                                      *(ushort *)
                                                       ((longlong)puVar3 +
                                                       (ulonglong)puVar3[uVar7 + 1]);
                                              if (uVar8 != 0) {
                                                plVar6 = (longlong *)
                                                         ((longlong)
                                                          ((longlong)puVar3 +
                                                          (ulonglong)puVar3[uVar7 + 1]) + 3);
                                                uVar9 = 0;
                                                do {
                                                  if (((*plVar6 != -0x724bde0defd224ab) ||
                                                      (plVar6[1] != -0x4ddc648a4ff5d960)) ||
                                                     ((plVar6[2] != 0x17f86227578d7956 ||
                                                      (bVar2 = false,
                                                      plVar6[3] != -0x2866c6493ba65e0d)))) {
                                                    bVar2 = true;
                                                  }
                                                  if (!bVar2) goto LAB_ram_0001fb60;
                                                  plVar6 = (longlong *)((longlong)plVar6 + 0x21);
                                                  uVar9 = uVar9 + 1;
                                                } while (uVar9 < uVar8);
                                              }
                                              uVar7 = uVar7 + 1;
                                            } while (uVar7 < uVar4);
                                          }
                                          if (((*param_3 != -0x5b792ad20f59a16d) ||
                                              (param_3[1] != -0x2c15f09912347a81)) ||
                                             ((param_3[2] != 0xccbecd97e436386 ||
                                              (bVar2 = false, param_3[3] != 0x3aa82a4cb9d28622)))) {
                                            bVar2 = true;
                                          }
                                          if (bVar2) {
                                            local_8 = 0x3aa82a4cb9d28622;
                                            local_10 = 0xccbecd97e436386;
                                            local_18 = 0xd3ea0f66edcb857f;
                                            local_20 = 0xa486d52df0a65e93;
                                            uVar7 = 0;
                                            do {
                                              uVar8 = (ulonglong)
                                                      *(ushort *)
                                                       ((longlong)puVar3 +
                                                       (ulonglong)puVar3[uVar7 + 1]);
                                              if (uVar8 != 0) {
                                                plVar6 = (longlong *)
                                                         ((longlong)
                                                          ((longlong)puVar3 +
                                                          (ulonglong)puVar3[uVar7 + 1]) + 3);
                                                uVar9 = 0;
                                                do {
                                                  if ((((*plVar6 != -0x5b792ad20f59a16d) ||
                                                       (plVar6[1] != -0x2c15f09912347a81)) ||
                                                      (plVar6[2] != 0xccbecd97e436386)) ||
                                                     (bVar2 = false, plVar6[3] != 0x3aa82a4cb9d28622
                                                     )) {
                                                    bVar2 = true;
                                                  }
                                                  if (!bVar2) goto LAB_ram_0001fb60;
                                                  plVar6 = (longlong *)((longlong)plVar6 + 0x21);
                                                  uVar9 = uVar9 + 1;
                                                } while (uVar9 < uVar8);
                                              }
                                              uVar7 = uVar7 + 1;
                                            } while (uVar7 < uVar4);
                                          }
                                          if (((*param_3 != 0x6d46af69e74bdfb4) ||
                                              (param_3[1] != -0xfb7c095e807c65c)) ||
                                             ((param_3[2] != -0x49badeacf352c099 ||
                                              (bVar2 = false, param_3[3] != 0x494ac5dec856a9e9)))) {
                                            bVar2 = true;
                                          }
                                          if (bVar2) {
                                            local_8 = 0x494ac5dec856a9e9;
                                            local_10 = 0xb64521530cad3f67;
                                            local_18 = 0xf0483f6a17f839a4;
                                            local_20 = 0x6d46af69e74bdfb4;
                                            uVar7 = 0;
                                            do {
                                              uVar8 = (ulonglong)
                                                      *(ushort *)
                                                       ((longlong)puVar3 +
                                                       (ulonglong)puVar3[uVar7 + 1]);
                                              if (uVar8 != 0) {
                                                plVar6 = (longlong *)
                                                         ((longlong)
                                                          ((longlong)puVar3 +
                                                          (ulonglong)puVar3[uVar7 + 1]) + 3);
                                                uVar9 = 0;
                                                do {
                                                  if (((*plVar6 != 0x6d46af69e74bdfb4) ||
                                                      (plVar6[1] != -0xfb7c095e807c65c)) ||
                                                     ((plVar6[2] != -0x49badeacf352c099 ||
                                                      (bVar2 = false,
                                                      plVar6[3] != 0x494ac5dec856a9e9)))) {
                                                    bVar2 = true;
                                                  }
                                                  if (!bVar2) goto LAB_ram_0001fb60;
                                                  plVar6 = (longlong *)((longlong)plVar6 + 0x21);
                                                  uVar9 = uVar9 + 1;
                                                } while (uVar9 < uVar8);
                                              }
                                              uVar7 = uVar7 + 1;
                                            } while (uVar7 < uVar4);
                                          }
                                          if ((((*param_3 != 0x602eddf9a6f50302) ||
                                               (param_3[1] != -0x30262668f50f63cf)) ||
                                              (param_3[2] != 0x5f9eab07c0325e41)) ||
                                             (bVar2 = false, param_3[3] != 0x74308a941848db62)) {
                                            bVar2 = true;
                                          }
                                          if (bVar2) {
                                            local_8 = 0x74308a941848db62;
                                            local_10 = 0x5f9eab07c0325e41;
                                            local_18 = 0xcfd9d9970af09c31;
                                            local_20 = 0x602eddf9a6f50302;
                                            uVar7 = 0;
                                            do {
                                              uVar8 = (ulonglong)
                                                      *(ushort *)
                                                       ((longlong)puVar3 +
                                                       (ulonglong)puVar3[uVar7 + 1]);
                                              if (uVar8 != 0) {
                                                plVar6 = (longlong *)
                                                         ((longlong)
                                                          ((longlong)puVar3 +
                                                          (ulonglong)puVar3[uVar7 + 1]) + 3);
                                                uVar9 = 0;
                                                do {
                                                  if (((*plVar6 != 0x602eddf9a6f50302) ||
                                                      (plVar6[1] != -0x30262668f50f63cf)) ||
                                                     ((plVar6[2] != 0x5f9eab07c0325e41 ||
                                                      (bVar2 = false,
                                                      plVar6[3] != 0x74308a941848db62)))) {
                                                    bVar2 = true;
                                                  }
                                                  if (!bVar2) goto LAB_ram_0001fb60;
                                                  plVar6 = (longlong *)((longlong)plVar6 + 0x21);
                                                  uVar9 = uVar9 + 1;
                                                } while (uVar9 < uVar8);
                                              }
                                              uVar7 = uVar7 + 1;
                                            } while (uVar7 < uVar4);
                                          }
                                          if (((*param_3 != -0x5a8406c1ee9740dd) ||
                                              (param_3[1] != 0x694d916b33d303ed)) ||
                                             ((param_3[2] != -0x5a746ce81742b801 ||
                                              (bVar2 = false, param_3[3] != 0x77eb01650c19b51)))) {
                                            bVar2 = true;
                                          }
                                          if (bVar2) {
                                            local_8 = 0x77eb01650c19b51;
                                            local_10 = 0xa58b9317e8bd47ff;
                                            local_18 = 0x694d916b33d303ed;
                                            local_20 = 0xa57bf93e1168bf23;
                                            uVar7 = 0;
                                            do {
                                              uVar8 = (ulonglong)
                                                      *(ushort *)
                                                       ((longlong)puVar3 +
                                                       (ulonglong)puVar3[uVar7 + 1]);
                                              if (uVar8 != 0) {
                                                plVar6 = (longlong *)
                                                         ((longlong)
                                                          ((longlong)puVar3 +
                                                          (ulonglong)puVar3[uVar7 + 1]) + 3);
                                                uVar9 = 0;
                                                do {
                                                  if ((((*plVar6 != -0x5a8406c1ee9740dd) ||
                                                       (plVar6[1] != 0x694d916b33d303ed)) ||
                                                      (plVar6[2] != -0x5a746ce81742b801)) ||
                                                     (bVar2 = false, plVar6[3] != 0x77eb01650c19b51)
                                                     ) {
                                                    bVar2 = true;
                                                  }
                                                  if (!bVar2) goto LAB_ram_0001fb60;
                                                  plVar6 = (longlong *)((longlong)plVar6 + 0x21);
                                                  uVar9 = uVar9 + 1;
                                                } while (uVar9 < uVar8);
                                              }
                                              uVar7 = uVar7 + 1;
                                            } while (uVar7 < uVar4);
                                          }
                                        }
                                        local_20 = 0;
                                        local_18 = 0;
                                        goto LAB_ram_0001ef00;
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    local_20 = 0;
    local_18 = 0;
  }
LAB_ram_0001ef00:
  *(undefined8 *)(param_1 + 2) = local_20;
  *(undefined8 *)(param_1 + 4) = local_18;
  *param_1 = 0;
  return;
LAB_ram_0001fb60:
  FUN_ram_0002df08(&local_20,0x1388000000000000,0,0x3e8000000000000);
  goto LAB_ram_0001ef00;
}

// Function: FUN_ram_0001fbf0
void FUN_ram_0001fbf0(undefined4 *param_1,undefined8 *param_2)

{
  ushort uVar1;
  ushort uVar2;
  bool bVar3;
  bool bVar4;
  char cVar5;
  ushort *puVar6;
  ushort *puVar7;
  ushort *puVar8;
  ulonglong uVar9;
  undefined8 local_18;
  undefined8 local_10;
  
  puVar6 = (ushort *)*param_2;
  uVar1 = *puVar6;
  if ((ulonglong)uVar1 == 0) {
    local_18 = 0;
    local_10 = 0;
  }
  else {
    bVar4 = false;
    uVar9 = 0;
    puVar8 = puVar6;
    do {
      puVar8 = puVar8 + 1;
      puVar7 = (ushort *)((longlong)puVar6 + (ulonglong)*puVar8);
      uVar2 = *puVar7;
      cVar5 = FUN_ram_000164d8((longlong)puVar7 + (ulonglong)uVar2 * 0x21 + 2,
                               (longlong)puVar7 + (ulonglong)uVar2 * 0x21 + 0x24,
                               *(undefined2 *)((longlong)puVar7 + (ulonglong)uVar2 * 0x21 + 0x22));
      if ((((*(longlong *)((longlong)puVar7 + (ulonglong)uVar2 * 0x21 + 2) != -0x7af703e2864bdf4) ||
           (*(longlong *)((longlong)puVar7 + (ulonglong)uVar2 * 0x21 + 10) != 0x2de7dd1cfc9a6d15))
          || (*(longlong *)((longlong)puVar7 + (ulonglong)uVar2 * 0x21 + 0x12) != 0x6bafec3babd968f6
             )) || (bVar3 = false,
                   *(longlong *)((longlong)puVar7 + (ulonglong)uVar2 * 0x21 + 0x1a) !=
                   -0x3726a59b99a8f2a9)) {
        bVar3 = true;
      }
      if (((!bVar3) || (bVar3 = bVar4, cVar5 != '\r')) && (bVar3 = true, bVar4)) {
        FUN_ram_0002df08(&local_18,0x190000000000000,0,0x3e8000000000000);
        goto LAB_ram_0001fe50;
      }
      bVar4 = bVar3;
      uVar9 = uVar9 + 1;
    } while (uVar9 < uVar1);
    local_18 = 0;
    local_10 = 0;
  }
LAB_ram_0001fe50:
  *(undefined8 *)(param_1 + 2) = local_18;
  *(undefined8 *)(param_1 + 4) = local_10;
  *param_1 = 0;
  return;
}

// Function: FUN_ram_0001fe70
void FUN_ram_0001fe70(undefined4 *param_1,undefined8 *param_2,longlong *param_3)

{
  ushort uVar1;
  bool bVar2;
  undefined1 uVar3;
  ushort *puVar4;
  ulonglong uVar5;
  ushort *puVar6;
  ulonglong *puVar7;
  ulonglong uVar8;
  longlong lVar9;
  longlong *plVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  char local_350 [424];
  longlong *local_1a8;
  ulonglong local_1a0;
  undefined8 local_198;
  undefined8 local_190;
  
  if ((((*param_3 != 0x6560b6dd6ee140db) || (param_3[1] != -0x163a020e81a9d7c4)) ||
      (param_3[2] != 0x10742a9290fc845b)) || (bVar2 = false, param_3[3] != 0x182e105eeb8708ad)) {
    bVar2 = true;
  }
  if (bVar2) {
    if (((*param_3 != -0x724bde0defd224ab) || (param_3[1] != -0x4ddc648a4ff5d960)) ||
       ((param_3[2] != 0x17f86227578d7956 || (bVar2 = false, param_3[3] != -0x2866c6493ba65e0d)))) {
      bVar2 = true;
    }
    if (bVar2) {
      if (((*param_3 != -0x5b792ad20f59a16d) || (param_3[1] != -0x2c15f09912347a81)) ||
         ((param_3[2] != 0xccbecd97e436386 || (bVar2 = false, param_3[3] != 0x3aa82a4cb9d28622)))) {
        bVar2 = true;
      }
      if (bVar2) {
        if ((((*param_3 != 0x6d46af69e74bdfb4) || (param_3[1] != -0xfb7c095e807c65c)) ||
            (param_3[2] != -0x49badeacf352c099)) ||
           (bVar2 = false, param_3[3] != 0x494ac5dec856a9e9)) {
          bVar2 = true;
        }
        if (bVar2) {
          if (((*param_3 != 0x602eddf9a6f50302) || (param_3[1] != -0x30262668f50f63cf)) ||
             ((param_3[2] != 0x5f9eab07c0325e41 || (bVar2 = false, param_3[3] != 0x74308a941848db62)
              ))) {
            bVar2 = true;
          }
          if (bVar2) {
            if (((*param_3 != -0x5a8406c1ee9740dd) || (param_3[1] != 0x694d916b33d303ed)) ||
               ((param_3[2] != -0x5a746ce81742b801 ||
                (bVar2 = false, param_3[3] != 0x77eb01650c19b51)))) {
              bVar2 = true;
            }
            local_1a8 = (longlong *)0x0;
            local_1a0 = 0;
            if (bVar2) goto LAB_ram_000216b0;
          }
        }
      }
    }
  }
  local_1a8 = (longlong *)0x0;
  puVar4 = (ushort *)*param_2;
  uVar5 = (ulonglong)*(ushort *)((longlong)puVar4 + param_2[1] + -2);
  uVar8 = (ulonglong)*puVar4;
  local_1a0 = 0;
  if (uVar8 <= uVar5) goto LAB_ram_000216b0;
  puVar6 = (ushort *)((longlong)puVar4 + (ulonglong)puVar4[uVar5 + 1]);
  uVar1 = *puVar6;
  if ((((*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 2) != 0x6ec031f25bd57904) ||
       (*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 10) != 0x71568ce6ec574ee)) ||
      (*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x518ef4a3deb2b1fd)) ||
     (bVar2 = false,
     *(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x1a) != -0x70ec43a95d324efe)) {
    bVar2 = true;
  }
  if (bVar2) {
    if (((*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 2) != 0x4873bce2144ae3b5) ||
        (*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 10) != -0x2911a2500a1ef197)) ||
       ((*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x60b8aa6da3403855 ||
        (bVar2 = false,
        *(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x1a) != 0x103cc0bd736050b0)))) {
      bVar2 = true;
    }
    if (bVar2) {
      if (((*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 2) != -0x44f118ed916356fa) ||
          (*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 10) != 0x6e904b4c145c1835)) ||
         ((*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x2a2f74470ab0ff18 ||
          (bVar2 = false,
          *(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x1a) != -0x2b367796f4eefba2)))
         ) {
        bVar2 = true;
      }
      if (bVar2) {
        if ((((*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 2) != 0x136d5ca2f1569155)
             || (*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 10) !=
                 0x340d9a0ae6f72a4f)) ||
            (*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x12) != -0x2a9d9b9ca96e3882
            )) || (bVar2 = false,
                  *(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                  0x698f3435f126add1)) {
          bVar2 = true;
        }
        local_1a8 = (longlong *)0x0;
        if (bVar2) goto LAB_ram_000216b0;
      }
    }
  }
  if (((*param_3 != 0x6560b6dd6ee140db) || (param_3[1] != -0x163a020e81a9d7c4)) ||
     ((param_3[2] != 0x10742a9290fc845b || (bVar2 = false, param_3[3] != 0x182e105eeb8708ad)))) {
    bVar2 = true;
  }
  if (bVar2) {
    local_190 = 0x182e105eeb8708ad;
    local_198 = 0x10742a9290fc845b;
    local_1a0 = 0xe9c5fdf17e56283c;
    local_1a8 = (longlong *)0x6560b6dd6ee140db;
    uVar5 = 0;
    do {
      uVar11 = (ulonglong)*(ushort *)((longlong)puVar4 + (ulonglong)puVar4[uVar5 + 1]);
      if (uVar11 != 0) {
        plVar10 = (longlong *)((longlong)((longlong)puVar4 + (ulonglong)puVar4[uVar5 + 1]) + 3);
        uVar12 = 0;
        do {
          if (((*plVar10 != 0x6560b6dd6ee140db) || (plVar10[1] != -0x163a020e81a9d7c4)) ||
             ((plVar10[2] != 0x10742a9290fc845b || (bVar2 = false, plVar10[3] != 0x182e105eeb8708ad)
              ))) {
            bVar2 = true;
          }
          if (!bVar2) goto LAB_ram_00021310;
          plVar10 = (longlong *)((longlong)plVar10 + 0x21);
          uVar12 = uVar12 + 1;
        } while (uVar12 < uVar11);
      }
      uVar5 = uVar5 + 1;
    } while (uVar5 < uVar8);
  }
  if ((((*param_3 != -0x724bde0defd224ab) || (param_3[1] != -0x4ddc648a4ff5d960)) ||
      (param_3[2] != 0x17f86227578d7956)) || (bVar2 = false, param_3[3] != -0x2866c6493ba65e0d)) {
    bVar2 = true;
  }
  if (bVar2) {
    local_190 = 0xd79939b6c459a1f3;
    local_198 = 0x17f86227578d7956;
    local_1a0 = 0xb2239b75b00a26a0;
    local_1a8 = (longlong *)0x8db421f2102ddb55;
    uVar5 = 0;
    do {
      uVar11 = (ulonglong)*(ushort *)((longlong)puVar4 + (ulonglong)puVar4[uVar5 + 1]);
      if (uVar11 != 0) {
        plVar10 = (longlong *)((longlong)((longlong)puVar4 + (ulonglong)puVar4[uVar5 + 1]) + 3);
        uVar12 = 0;
        do {
          if (((*plVar10 != -0x724bde0defd224ab) || (plVar10[1] != -0x4ddc648a4ff5d960)) ||
             ((plVar10[2] != 0x17f86227578d7956 ||
              (bVar2 = false, plVar10[3] != -0x2866c6493ba65e0d)))) {
            bVar2 = true;
          }
          if (!bVar2) goto LAB_ram_00021310;
          plVar10 = (longlong *)((longlong)plVar10 + 0x21);
          uVar12 = uVar12 + 1;
        } while (uVar12 < uVar11);
      }
      uVar5 = uVar5 + 1;
    } while (uVar5 < uVar8);
  }
  if (((*param_3 != -0x5b792ad20f59a16d) || (param_3[1] != -0x2c15f09912347a81)) ||
     ((param_3[2] != 0xccbecd97e436386 || (bVar2 = false, param_3[3] != 0x3aa82a4cb9d28622)))) {
    bVar2 = true;
  }
  if (bVar2) {
    local_190 = 0x3aa82a4cb9d28622;
    local_198 = 0xccbecd97e436386;
    local_1a0 = 0xd3ea0f66edcb857f;
    local_1a8 = (longlong *)0xa486d52df0a65e93;
    uVar5 = 0;
    do {
      uVar11 = (ulonglong)*(ushort *)((longlong)puVar4 + (ulonglong)puVar4[uVar5 + 1]);
      if (uVar11 != 0) {
        plVar10 = (longlong *)((longlong)((longlong)puVar4 + (ulonglong)puVar4[uVar5 + 1]) + 3);
        uVar12 = 0;
        do {
          if ((((*plVar10 != -0x5b792ad20f59a16d) || (plVar10[1] != -0x2c15f09912347a81)) ||
              (plVar10[2] != 0xccbecd97e436386)) ||
             (bVar2 = false, plVar10[3] != 0x3aa82a4cb9d28622)) {
            bVar2 = true;
          }
          if (!bVar2) goto LAB_ram_00021310;
          plVar10 = (longlong *)((longlong)plVar10 + 0x21);
          uVar12 = uVar12 + 1;
        } while (uVar12 < uVar11);
      }
      uVar5 = uVar5 + 1;
    } while (uVar5 < uVar8);
  }
  if (((*param_3 != 0x6d46af69e74bdfb4) || (param_3[1] != -0xfb7c095e807c65c)) ||
     ((param_3[2] != -0x49badeacf352c099 || (bVar2 = false, param_3[3] != 0x494ac5dec856a9e9)))) {
    bVar2 = true;
  }
  if (bVar2) {
    local_190 = 0x494ac5dec856a9e9;
    local_198 = 0xb64521530cad3f67;
    local_1a0 = 0xf0483f6a17f839a4;
    local_1a8 = (longlong *)0x6d46af69e74bdfb4;
    uVar5 = 0;
    do {
      uVar11 = (ulonglong)*(ushort *)((longlong)puVar4 + (ulonglong)puVar4[uVar5 + 1]);
      if (uVar11 != 0) {
        plVar10 = (longlong *)((longlong)((longlong)puVar4 + (ulonglong)puVar4[uVar5 + 1]) + 3);
        uVar12 = 0;
        do {
          if (((*plVar10 != 0x6d46af69e74bdfb4) || (plVar10[1] != -0xfb7c095e807c65c)) ||
             ((plVar10[2] != -0x49badeacf352c099 ||
              (bVar2 = false, plVar10[3] != 0x494ac5dec856a9e9)))) {
            bVar2 = true;
          }
          if (!bVar2) goto LAB_ram_00021310;
          plVar10 = (longlong *)((longlong)plVar10 + 0x21);
          uVar12 = uVar12 + 1;
        } while (uVar12 < uVar11);
      }
      uVar5 = uVar5 + 1;
    } while (uVar5 < uVar8);
  }
  if ((((*param_3 != 0x602eddf9a6f50302) || (param_3[1] != -0x30262668f50f63cf)) ||
      (param_3[2] != 0x5f9eab07c0325e41)) || (bVar2 = false, param_3[3] != 0x74308a941848db62)) {
    bVar2 = true;
  }
  if (bVar2) {
    local_190 = 0x74308a941848db62;
    local_198 = 0x5f9eab07c0325e41;
    local_1a0 = 0xcfd9d9970af09c31;
    local_1a8 = (longlong *)0x602eddf9a6f50302;
    uVar5 = 0;
    do {
      uVar11 = (ulonglong)*(ushort *)((longlong)puVar4 + (ulonglong)puVar4[uVar5 + 1]);
      if (uVar11 != 0) {
        plVar10 = (longlong *)((longlong)((longlong)puVar4 + (ulonglong)puVar4[uVar5 + 1]) + 3);
        uVar12 = 0;
        do {
          if (((*plVar10 != 0x602eddf9a6f50302) || (plVar10[1] != -0x30262668f50f63cf)) ||
             ((plVar10[2] != 0x5f9eab07c0325e41 || (bVar2 = false, plVar10[3] != 0x74308a941848db62)
              ))) {
            bVar2 = true;
          }
          if (!bVar2) goto LAB_ram_00021310;
          plVar10 = (longlong *)((longlong)plVar10 + 0x21);
          uVar12 = uVar12 + 1;
        } while (uVar12 < uVar11);
      }
      uVar5 = uVar5 + 1;
    } while (uVar5 < uVar8);
  }
  if (((*param_3 != -0x5a8406c1ee9740dd) || (param_3[1] != 0x694d916b33d303ed)) ||
     ((param_3[2] != -0x5a746ce81742b801 || (bVar2 = false, param_3[3] != 0x77eb01650c19b51)))) {
    bVar2 = true;
  }
  if (bVar2) {
    local_190 = 0x77eb01650c19b51;
    local_198 = 0xa58b9317e8bd47ff;
    local_1a0 = 0x694d916b33d303ed;
    local_1a8 = (longlong *)0xa57bf93e1168bf23;
    uVar5 = 0;
    do {
      uVar11 = (ulonglong)*(ushort *)((longlong)puVar4 + (ulonglong)puVar4[uVar5 + 1]);
      if (uVar11 != 0) {
        plVar10 = (longlong *)((longlong)((longlong)puVar4 + (ulonglong)puVar4[uVar5 + 1]) + 3);
        uVar12 = 0;
        do {
          if ((((*plVar10 != -0x5a8406c1ee9740dd) || (plVar10[1] != 0x694d916b33d303ed)) ||
              (plVar10[2] != -0x5a746ce81742b801)) ||
             (bVar2 = false, plVar10[3] != 0x77eb01650c19b51)) {
            bVar2 = true;
          }
          if (!bVar2) goto LAB_ram_00021310;
          plVar10 = (longlong *)((longlong)plVar10 + 0x21);
          uVar12 = uVar12 + 1;
        } while (uVar12 < uVar11);
      }
      uVar5 = uVar5 + 1;
    } while (uVar5 < uVar8);
    local_1a8 = (longlong *)0x0;
    local_1a0 = 0;
    goto LAB_ram_000216b0;
  }
  goto LAB_ram_00021518;
LAB_ram_00021310:
  if (((*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 2) != 0x6ec031f25bd57904) ||
      (*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 10) != 0x71568ce6ec574ee)) ||
     ((*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x518ef4a3deb2b1fd ||
      (bVar2 = false,
      *(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x1a) != -0x70ec43a95d324efe)))) {
    bVar2 = true;
  }
  if (bVar2) {
    if (((*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 2) != 0x4873bce2144ae3b5) ||
        (*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 10) != -0x2911a2500a1ef197)) ||
       ((*(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x60b8aa6da3403855 ||
        (bVar2 = false,
        *(longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x1a) != 0x103cc0bd736050b0)))) {
      bVar2 = true;
    }
    local_1a8 = (longlong *)0x0;
    local_1a0 = 0;
    if ((bVar2) ||
       (uVar5 = (ulonglong)*(ushort *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x22),
       uVar5 < 0xc)) goto LAB_ram_000216b0;
    plVar10 = (longlong *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x24);
    lVar9 = *plVar10;
    uVar3 = 0;
    if (lVar9 != -0x77a4a414b3c0b4bf) {
      if (lVar9 == 0x65879cc54d18aca8) {
        uVar3 = 2;
      }
      else {
        if (lVar9 != -0x37788a1e6e613908) goto LAB_ram_000216b0;
        uVar3 = 1;
      }
    }
    local_198 = CONCAT44(CONCAT31(local_198._5_3_,uVar3),
                         *(undefined4 *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x2c));
    local_1a8 = plVar10;
    local_1a0 = uVar5;
    lVar9 = FUN_ram_000133b8(&local_1a8);
    plVar10 = local_1a8;
    uVar5 = local_1a0;
    goto joined_r0x000215e8;
  }
  FUN_ram_000149e0(local_350,(longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x24,
                   *(undefined2 *)((longlong)puVar6 + (ulonglong)uVar1 * 0x21 + 0x22));
  if (local_350[0] != '\x02') {
    FUN_ram_00031b28(&local_1a8,local_350,0x1a8);
    lVar9 = FUN_ram_000159c0(&local_1a8);
    plVar10 = local_1a8;
    uVar5 = local_1a0;
    if ((char)local_1a8 == '\0') {
      if ((int)local_1a0 != 0) {
        puVar7 = &local_1a0;
        goto LAB_ram_000215d0;
      }
    }
    else if ((int)local_198 != 0) {
      puVar7 = &local_198;
LAB_ram_000215d0:
      *(undefined4 *)puVar7 = 0;
      uVar5 = local_1a0;
    }
joined_r0x000215e8:
    local_1a0 = 0;
    local_1a8 = (longlong *)0x0;
    if (lVar9 != 0) {
      local_1a8 = plVar10;
      local_1a0 = uVar5;
      FUN_ram_00000908(&local_1a8,0x1388000000000000,0,0x3e8000000000000,0);
    }
    goto LAB_ram_000216b0;
  }
LAB_ram_00021518:
  local_1a8 = (longlong *)0x0;
  local_1a0 = 0;
LAB_ram_000216b0:
  *(longlong **)(param_1 + 2) = local_1a8;
  *(ulonglong *)(param_1 + 4) = local_1a0;
  *param_1 = 0;
  return;
}

// Function: FUN_ram_000216f0
void FUN_ram_000216f0(undefined4 *param_1,undefined8 *param_2,longlong *param_3,longlong *param_4,
                     longlong *param_5)

{
  char *pcVar1;
  bool bVar2;
  bool bVar3;
  byte bVar4;
  ushort *puVar5;
  ulonglong uVar6;
  longlong lVar7;
  ulonglong uVar8;
  ulonglong *puVar9;
  longlong *plVar10;
  ulonglong uVar11;
  undefined8 uVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  longlong lVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  longlong local_570;
  longlong local_568;
  longlong local_560;
  longlong local_558;
  longlong local_550;
  longlong local_548;
  longlong local_540;
  longlong local_538;
  uint local_530;
  undefined4 uStack_52c;
  ulonglong local_528 [2];
  char acStack_518 [880];
  longlong *local_1a8;
  ulonglong local_1a0;
  int local_198;
  undefined1 local_194;
  
  if ((((*param_3 != 0x6560b6dd6ee140db) || (param_3[1] != -0x163a020e81a9d7c4)) ||
      (param_3[2] != 0x10742a9290fc845b)) || (bVar2 = false, param_3[3] != 0x182e105eeb8708ad)) {
    bVar2 = true;
  }
  if (bVar2) {
    if (((*param_3 != -0x724bde0defd224ab) || (param_3[1] != -0x4ddc648a4ff5d960)) ||
       ((param_3[2] != 0x17f86227578d7956 || (bVar2 = false, param_3[3] != -0x2866c6493ba65e0d)))) {
      bVar2 = true;
    }
    if (bVar2) {
      if (((*param_3 != -0x5b792ad20f59a16d) || (param_3[1] != -0x2c15f09912347a81)) ||
         ((param_3[2] != 0xccbecd97e436386 || (bVar2 = false, param_3[3] != 0x3aa82a4cb9d28622)))) {
        bVar2 = true;
      }
      if (bVar2) {
        if ((((*param_3 != 0x6d46af69e74bdfb4) || (param_3[1] != -0xfb7c095e807c65c)) ||
            (param_3[2] != -0x49badeacf352c099)) ||
           (bVar2 = false, param_3[3] != 0x494ac5dec856a9e9)) {
          bVar2 = true;
        }
        if (bVar2) {
          if (((*param_3 != 0x602eddf9a6f50302) || (param_3[1] != -0x30262668f50f63cf)) ||
             ((param_3[2] != 0x5f9eab07c0325e41 || (bVar2 = false, param_3[3] != 0x74308a941848db62)
              ))) {
            bVar2 = true;
          }
          if (bVar2) {
            if (((*param_3 != -0x5a8406c1ee9740dd) || (param_3[1] != 0x694d916b33d303ed)) ||
               ((param_3[2] != -0x5a746ce81742b801 ||
                (bVar2 = false, param_3[3] != 0x77eb01650c19b51)))) {
              bVar2 = true;
            }
            uVar12 = 0;
            uVar15 = 0;
            if (bVar2) goto LAB_ram_00022be8;
          }
        }
      }
    }
  }
  uVar12 = 0;
  puVar5 = (ushort *)*param_2;
  uVar6 = (ulonglong)*(ushort *)((longlong)puVar5 + param_2[1] + -2);
  uVar15 = 0;
  if (*puVar5 <= uVar6) goto LAB_ram_00022be8;
  puVar5 = (ushort *)((longlong)puVar5 + (ulonglong)puVar5[uVar6 + 1]);
  uVar6 = (ulonglong)*puVar5;
  if ((((*(longlong *)((longlong)puVar5 + uVar6 * 0x21 + 2) != 0x6ec031f25bd57904) ||
       (*(longlong *)((longlong)puVar5 + uVar6 * 0x21 + 10) != 0x71568ce6ec574ee)) ||
      (*(longlong *)((longlong)puVar5 + uVar6 * 0x21 + 0x12) != 0x518ef4a3deb2b1fd)) ||
     (bVar2 = false, *(longlong *)((longlong)puVar5 + uVar6 * 0x21 + 0x1a) != -0x70ec43a95d324efe))
  {
    bVar2 = true;
  }
  uVar13 = (ulonglong)*(ushort *)((longlong)puVar5 + uVar6 * 0x21 + 0x22);
  plVar10 = (longlong *)((longlong)puVar5 + uVar6 * 0x21 + 0x24);
  if (bVar2) {
    if (((*(longlong *)((longlong)puVar5 + uVar6 * 0x21 + 2) != 0x4873bce2144ae3b5) ||
        (*(longlong *)((longlong)puVar5 + uVar6 * 0x21 + 10) != -0x2911a2500a1ef197)) ||
       ((*(longlong *)((longlong)puVar5 + uVar6 * 0x21 + 0x12) != 0x60b8aa6da3403855 ||
        (bVar2 = false, *(longlong *)((longlong)puVar5 + uVar6 * 0x21 + 0x1a) != 0x103cc0bd736050b0)
        ))) {
      bVar2 = true;
    }
    uVar12 = 0;
    uVar15 = 0;
    if (bVar2) goto LAB_ram_00022be8;
    bVar4 = 2;
    if (0xb < uVar13) {
      lVar7 = *plVar10;
      if (lVar7 != -0x77a4a414b3c0b4bf) {
        if (lVar7 == 0x65879cc54d18aca8) {
          uVar12 = 2;
        }
        else {
          if (lVar7 != -0x37788a1e6e613908) goto LAB_ram_00022368;
          uVar12 = 1;
        }
      }
      local_198 = *(int *)((longlong)puVar5 + uVar6 * 0x21 + 0x2c);
      local_194 = (undefined1)uVar12;
      local_1a8 = plVar10;
      local_1a0 = uVar13;
      FUN_ram_00013658(&local_530,&local_1a8);
      puVar9 = local_528;
      lVar7 = 0;
      do {
        lVar16 = lVar7;
        if ((ulonglong)local_530 * 0x38 == lVar16) {
          bVar4 = 2;
          goto LAB_ram_00022368;
        }
        pcVar1 = (char *)((longlong)puVar9 + 0x33);
        puVar9 = puVar9 + 7;
        lVar7 = lVar16 + 0x38;
      } while (*pcVar1 != '(');
      bVar4 = 2;
      if ((acStack_518[lVar16] == '\x01') && (bVar4 = 1, acStack_518[lVar16 + 1] == '\0')) {
        bVar4 = 0;
      }
    }
    goto LAB_ram_00022368;
  }
  if (uVar6 == 0) {
    uVar12 = 0;
    goto LAB_ram_00022be8;
  }
  uVar8 = 0;
  uVar17 = 0;
  uVar14 = 0;
  do {
    if (((*(longlong *)((longlong)puVar5 + 3) != 0x50126c1f9cda329f) ||
        (*(longlong *)((longlong)puVar5 + 0xb) != 0x67c3ec2339739bc0)) ||
       ((*(longlong *)((longlong)puVar5 + 0x13) != 0x1c9c6ad4e763280d ||
        (bVar2 = false, *(longlong *)((longlong)puVar5 + 0x1b) != 0x57fed5d0bbfd7df4)))) {
      bVar2 = true;
    }
    if (bVar2) {
      if (((*(longlong *)((longlong)puVar5 + 3) != -0x69f4a935eeabd322) ||
          (*(longlong *)((longlong)puVar5 + 0xb) != 0x156492098a673ea6)) ||
         ((*(longlong *)((longlong)puVar5 + 0x13) != -0x58a98d91a3c1cd4c ||
          (bVar2 = false, *(longlong *)((longlong)puVar5 + 0x1b) != -0x3eac325f67561f09)))) {
        bVar2 = true;
      }
      if (!bVar2) {
        uVar15 = 1;
        uVar19 = uVar8;
        if ((uVar8 & uVar17) == 0) goto LAB_ram_00021d68;
LAB_ram_00022070:
        uVar17 = 1;
        uVar11 = 1;
        uVar18 = 1;
        break;
      }
      if ((((*(longlong *)((longlong)puVar5 + 3) != 0x70445cd3ea4a744d) ||
           (*(longlong *)((longlong)puVar5 + 0xb) != 0x6721023cd0ded5e7)) ||
          (*(longlong *)((longlong)puVar5 + 0x13) != 0x158269176877d439)) ||
         (bVar2 = false, *(longlong *)((longlong)puVar5 + 0x1b) != -0x25fedc0091beafba)) {
        bVar2 = true;
      }
      uVar19 = uVar8 & uVar15;
      if (bVar2) {
        if ((uVar19 & uVar17) != 0) goto LAB_ram_00022070;
        uVar17 = (uVar19 ^ 1) & uVar17;
      }
      else {
        uVar17 = 1;
        uVar11 = 1;
        uVar18 = 1;
        if (uVar19 != 0) break;
      }
    }
    else {
      uVar8 = 1;
      uVar19 = uVar15;
      if ((uVar15 & uVar17) != 0) goto LAB_ram_00022070;
LAB_ram_00021d68:
      uVar17 = (uVar19 ^ 1) & uVar17;
    }
    uVar11 = uVar15;
    puVar5 = (ushort *)((longlong)puVar5 + 0x21);
    uVar14 = uVar14 + 1;
    uVar15 = uVar11;
    uVar18 = uVar8;
  } while (uVar14 < uVar6);
  uVar12 = 0;
  if (uVar18 == 0) {
    uVar15 = 0;
    goto LAB_ram_00022be8;
  }
  if (uVar11 == 0) {
    uVar15 = 0;
    goto LAB_ram_00022be8;
  }
  uVar15 = 0;
  if (uVar17 == 0) goto LAB_ram_00022be8;
  FUN_ram_000149e0(&local_530,plVar10,uVar13);
  if ((char)local_530 == '\x02') {
    uVar12 = 0;
    uVar15 = 0;
    goto LAB_ram_00022be8;
  }
  FUN_ram_00031b28(&local_1a8,&local_530,0x1a8);
  bVar4 = FUN_ram_00015780(&local_1a8,0x75);
  if ((char)local_1a8 == '\0') {
    if ((int)local_1a0 != 0) {
      puVar9 = &local_1a0;
      goto LAB_ram_00022348;
    }
  }
  else if (local_198 != 0) {
    puVar9 = (ulonglong *)&local_198;
LAB_ram_00022348:
    *(int *)puVar9 = 0;
  }
LAB_ram_00022368:
  uVar12 = 0;
  uVar15 = 0;
  if (bVar4 == 2) goto LAB_ram_00022be8;
  if ((bVar4 & 1) == 0) {
    local_558 = 0x615d2f450302a67c;
    local_560 = -0x1b1f092d3d1b444f;
    local_568 = 0x3174c9ab6af3653d;
    local_570 = 0x3aaddbbef37afac6;
    local_550 = -0x7b7e5401a87764fa;
    local_548 = 0x35c01846637f68fb;
    local_540 = 0x553beb1adc39c4da;
    local_538 = 0x100000000f0a098;
    if (*param_4 == -0x7b7e5401a87764fa) goto LAB_ram_00022560;
LAB_ram_000225b0:
    bVar2 = true;
  }
  else {
    local_558 = 0x100000000f0a098;
    local_560 = 0x553beb1adc39c4da;
    local_568 = 0x35c01846637f68fb;
    local_570 = -0x7b7e5401a87764fa;
    local_550 = 0x3aaddbbef37afac6;
    local_548 = 0x3174c9ab6af3653d;
    local_540 = -0x1b1f092d3d1b444f;
    local_538 = 0x615d2f450302a67c;
    if (*param_4 != 0x3aaddbbef37afac6) goto LAB_ram_000225b0;
LAB_ram_00022560:
    if (((local_548 != param_4[1]) || (local_540 != param_4[2])) ||
       (bVar2 = false, local_538 != param_4[3])) goto LAB_ram_000225b0;
  }
  if (bVar2) {
    if (((local_550 != 0x3aaddbbef37afac6) || (local_548 != 0x3174c9ab6af3653d)) ||
       ((local_540 != -0x1b1f092d3d1b444f || (bVar2 = false, local_538 != 0x615d2f450302a67c)))) {
      bVar2 = true;
    }
    if (bVar2) {
      if (((local_550 != 0x27b2edaf600e01ce) || (local_548 != 0x5a14542f1963bd17)) ||
         ((local_540 != -0x382d7d44cca569c1 || (bVar3 = false, local_538 != 0x6482201eceb29e02)))) {
        bVar3 = true;
      }
      bVar2 = true;
      if (bVar3) goto LAB_ram_00022860;
    }
    if ((((*param_4 != 0x3aaddbbef37afac6) || (param_4[1] != 0x3174c9ab6af3653d)) ||
        (param_4[2] != -0x1b1f092d3d1b444f)) || (bVar2 = false, param_4[3] != 0x615d2f450302a67c)) {
      bVar2 = true;
    }
    if (bVar2) {
      if (((*param_4 != 0x27b2edaf600e01ce) || (param_4[1] != 0x5a14542f1963bd17)) ||
         ((param_4[2] != -0x382d7d44cca569c1 || (bVar3 = false, param_4[3] != 0x6482201eceb29e02))))
      {
        bVar3 = true;
      }
      bVar2 = false;
      if (!bVar3) goto LAB_ram_00022860;
      bVar2 = true;
      lVar7 = *param_5;
    }
    else {
      bVar2 = false;
      lVar7 = *param_5;
    }
    if (local_570 != lVar7) goto LAB_ram_000228f0;
LAB_ram_00022878:
    if (((local_568 != param_5[1]) || (local_560 != param_5[2])) ||
       (bVar3 = false, local_558 != param_5[3])) goto LAB_ram_000228f0;
  }
  else {
    bVar2 = false;
LAB_ram_00022860:
    if (local_570 == *param_5) goto LAB_ram_00022878;
LAB_ram_000228f0:
    bVar3 = true;
  }
  if (bVar3) {
    if ((((local_570 != 0x3aaddbbef37afac6) || (local_568 != 0x3174c9ab6af3653d)) ||
        (local_560 != -0x1b1f092d3d1b444f)) || (bVar3 = false, local_558 != 0x615d2f450302a67c)) {
      bVar3 = true;
    }
    if (bVar3) {
      if (((local_570 != 0x27b2edaf600e01ce) || (local_568 != 0x5a14542f1963bd17)) ||
         ((local_560 != -0x382d7d44cca569c1 || (bVar3 = false, local_558 != 0x6482201eceb29e02)))) {
        bVar3 = true;
      }
      uVar12 = 0;
      if (bVar3) goto LAB_ram_00022be8;
    }
    if (((*param_5 != 0x3aaddbbef37afac6) || (param_5[1] != 0x3174c9ab6af3653d)) ||
       ((param_5[2] != -0x1b1f092d3d1b444f || (bVar3 = false, param_5[3] != 0x615d2f450302a67c)))) {
      bVar3 = true;
    }
    if (bVar3) {
      if ((((*param_5 != 0x27b2edaf600e01ce) || (param_5[1] != 0x5a14542f1963bd17)) ||
          (param_5[2] != -0x382d7d44cca569c1)) || (bVar4 = 0, param_5[3] != 0x6482201eceb29e02)) {
        bVar4 = 1;
      }
      bVar2 = (bool)(bVar2 | bVar4);
    }
  }
  uVar12 = 0;
  if (!bVar2) {
    FUN_ram_0002df08(&local_530,0x4e7000000000000,0,0x3e8000000000000);
    uVar12 = CONCAT44(uStack_52c,local_530);
    uVar15 = local_528[0];
  }
LAB_ram_00022be8:
  *(undefined8 *)(param_1 + 2) = uVar12;
  *(ulonglong *)(param_1 + 4) = uVar15;
  *param_1 = 0;
  return;
}

// Function: FUN_ram_00022c08
void FUN_ram_00022c08(undefined4 *param_1,undefined8 *param_2,longlong *param_3)

{
  ushort uVar1;
  bool bVar2;
  ushort *puVar3;
  ushort *puVar4;
  longlong *plVar5;
  ulonglong uVar6;
  undefined8 uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  undefined8 uVar10;
  undefined8 local_18;
  undefined8 local_10;
  
  if ((((*param_3 != 0x6560b6dd6ee140db) || (param_3[1] != -0x163a020e81a9d7c4)) ||
      (param_3[2] != 0x10742a9290fc845b)) || (bVar2 = false, param_3[3] != 0x182e105eeb8708ad)) {
    bVar2 = true;
  }
  if (bVar2) {
    if (((*param_3 != -0x724bde0defd224ab) || (param_3[1] != -0x4ddc648a4ff5d960)) ||
       ((param_3[2] != 0x17f86227578d7956 || (bVar2 = false, param_3[3] != -0x2866c6493ba65e0d)))) {
      bVar2 = true;
    }
    if (bVar2) {
      if (((*param_3 != -0x5b792ad20f59a16d) || (param_3[1] != -0x2c15f09912347a81)) ||
         ((param_3[2] != 0xccbecd97e436386 || (bVar2 = false, param_3[3] != 0x3aa82a4cb9d28622)))) {
        bVar2 = true;
      }
      if (bVar2) {
        if ((((*param_3 != 0x6d46af69e74bdfb4) || (param_3[1] != -0xfb7c095e807c65c)) ||
            (param_3[2] != -0x49badeacf352c099)) ||
           (bVar2 = false, param_3[3] != 0x494ac5dec856a9e9)) {
          bVar2 = true;
        }
        if (bVar2) {
          if (((*param_3 != 0x602eddf9a6f50302) || (param_3[1] != -0x30262668f50f63cf)) ||
             ((param_3[2] != 0x5f9eab07c0325e41 || (bVar2 = false, param_3[3] != 0x74308a941848db62)
              ))) {
            bVar2 = true;
          }
          if (bVar2) {
            if (((*param_3 != -0x5a8406c1ee9740dd) || (param_3[1] != 0x694d916b33d303ed)) ||
               ((param_3[2] != -0x5a746ce81742b801 ||
                (bVar2 = false, param_3[3] != 0x77eb01650c19b51)))) {
              bVar2 = true;
            }
            uVar7 = 0;
            uVar10 = 0;
            if (bVar2) goto LAB_ram_00023be8;
          }
        }
      }
    }
  }
  puVar3 = (ushort *)*param_2;
  uVar8 = (ulonglong)*(ushort *)((longlong)puVar3 + param_2[1] + -2);
  uVar7 = 0;
  uVar10 = 0;
  if (uVar8 < *puVar3) {
    puVar4 = (ushort *)((longlong)puVar3 + (ulonglong)puVar3[uVar8 + 1]);
    uVar1 = *puVar4;
    plVar5 = (longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 2);
    if ((((*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 2) != 0x4873bce2144ae3b5) ||
         (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) != -0x2911a2500a1ef197)) ||
        (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x60b8aa6da3403855)) ||
       (bVar2 = false,
       *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) != 0x103cc0bd736050b0)) {
      bVar2 = true;
    }
    if (bVar2) {
      if (((*plVar5 != -0x1e8395f2e7b51c4b) ||
          (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) != -0x51f325fec501496b))
         || ((*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x98144e7e5ae3fa8
             || (bVar2 = false,
                *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                0x40ee2497930cf7ea)))) {
        bVar2 = true;
      }
      if (bVar2) {
        if (((*plVar5 != 0x6ec031f25bd57904) ||
            (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) != 0x71568ce6ec574ee))
           || ((*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                0x518ef4a3deb2b1fd ||
               (bVar2 = false,
               *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
               -0x70ec43a95d324efe)))) {
          bVar2 = true;
        }
        if (bVar2) {
          if ((((*plVar5 != 0x715b8f7af9be1205) ||
               (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                -0x3fbd123929120c83)) ||
              (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
               -0x1178411a20edb01e)) ||
             (bVar2 = false,
             *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) != -0x4693a2c08ba113c1
             )) {
            bVar2 = true;
          }
          if (bVar2) {
            if (((*plVar5 != -0x3b66289859b23cf6) ||
                (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                 0x75b1926ae1365115)) ||
               ((*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                 0x678ad2090231d088 ||
                (bVar2 = false,
                *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                -0x139993aed94b961d)))) {
              bVar2 = true;
            }
            if (bVar2) {
              if (((*plVar5 != 0x136d5ca2f1569155) ||
                  (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                   0x340d9a0ae6f72a4f)) ||
                 ((*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                   -0x2a9d9b9ca96e3882 ||
                  (bVar2 = false,
                  *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                  0x698f3435f126add1)))) {
                bVar2 = true;
              }
              if (bVar2) {
                if ((((*plVar5 != -0x16a608d8d48b0286) ||
                     (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                      0x7a819dd33c7070c6)) ||
                    (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                     0x6dd2523bce0a93a0)) ||
                   (bVar2 = false,
                   *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                   -0x2c4478dc22ab5fac)) {
                  bVar2 = true;
                }
                if (bVar2) {
                  if (((*plVar5 != -0x44f118ed916356fa) ||
                      (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                       0x6e904b4c145c1835)) ||
                     ((*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                       0x2a2f74470ab0ff18 ||
                      (bVar2 = false,
                      *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                      -0x2b367796f4eefba2)))) {
                    bVar2 = true;
                  }
                  if (bVar2) {
                    if (((*plVar5 != -0x4fc4eec7e6cb4135) ||
                        (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                         0x45acad558b7e296b)) ||
                       ((*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                         0x59369b4a1734ee6f ||
                        (bVar2 = false,
                        *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                        0x42c79970523f5e6b)))) {
                      bVar2 = true;
                    }
                    if (bVar2) {
                      if ((((*plVar5 != -0x1d323195ffe246f3) ||
                           (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                            0x67889bcdcd17de84)) ||
                          (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                           0x5666dfd02b922d2b)) ||
                         (bVar2 = false,
                         *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                         0x548b03e01a423aa3)) {
                        bVar2 = true;
                      }
                      if (bVar2) {
                        if (((*plVar5 != -0x6c2c22b8abad132c) ||
                            (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                             0x1776bd19d4d98a5b)) ||
                           ((*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                             0x6f034a62de39afcb ||
                            (bVar2 = false,
                            *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                            -0x5f19bd0c7dda6fc5)))) {
                          bVar2 = true;
                        }
                        if (bVar2) {
                          if (((*plVar5 != -0x1bb09aaaa3eacf65) ||
                              (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                               0x6493c705f351bd52)) ||
                             ((*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                               0x262c1d3289763901 ||
                              (bVar2 = false,
                              *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                              0x5be22f238cb47253)))) {
                            bVar2 = true;
                          }
                          if (bVar2) {
                            if ((((*plVar5 != -0x7af703e2864bdf4) ||
                                 (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                                  0x2de7dd1cfc9a6d15)) ||
                                (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                 0x6bafec3babd968f6)) ||
                               (bVar2 = false,
                               *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                               -0x3726a59b99a8f2a9)) {
                              bVar2 = true;
                            }
                            if (bVar2) {
                              if (((*plVar5 != -0x372c55a8b3c334fc) ||
                                  (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                                   0x72e40dd1add9f2d5)) ||
                                 ((*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12)
                                   != 0x42e6fdaa3eff7804 ||
                                  (bVar2 = false,
                                  *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a)
                                  != -0x3a991ec56a126c8d)))) {
                                bVar2 = true;
                              }
                              if (bVar2) {
                                if (((*plVar5 != -0xc5e8ffce1a16dfa) ||
                                    (*(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10)
                                     != -0x2070af22c1e0392a)) ||
                                   ((*(longlong *)
                                      ((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                     -0x4d27c110388d62ba ||
                                    (bVar2 = false,
                                    *(longlong *)((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a)
                                    != -0x19ea30d62c1318f6)))) {
                                  bVar2 = true;
                                }
                                if (bVar2) {
                                  if ((((*plVar5 != -0x241f8dfce1a16dfa) ||
                                       (*(longlong *)
                                         ((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                                        0x77ca68769172b20b)) ||
                                      (*(longlong *)
                                        ((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                       0x533f7524d0ace446)) ||
                                     (bVar2 = false,
                                     *(longlong *)
                                      ((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                     -0x6567b74b076d7538)) {
                                    bVar2 = true;
                                  }
                                  if (bVar2) {
                                    if (((*plVar5 != 0x1be9073efd071895) ||
                                        (*(longlong *)
                                          ((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                                         0x103eb598830568a5)) ||
                                       ((*(longlong *)
                                          ((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                         -0x6f5cf633300ceda6 ||
                                        (bVar2 = false,
                                        *(longlong *)
                                         ((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                        -0x5ca0aa4a02280026)))) {
                                      bVar2 = true;
                                    }
                                    if (bVar2) {
                                      if (((*plVar5 != -0x2d9d51bf92ab29f4) ||
                                          (*(longlong *)
                                            ((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                                           0x45eefdfe7495b816)) ||
                                         ((*(longlong *)
                                            ((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                           0xbb5f49c7d946b85 ||
                                          (bVar2 = false,
                                          *(longlong *)
                                           ((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                          0x115c61a060bb5829)))) {
                                        bVar2 = true;
                                      }
                                      if (bVar2) {
                                        if ((((*plVar5 != 0x366a33d8ef74db2b) ||
                                             (*(longlong *)
                                               ((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 10) !=
                                              0x6819eac7d96353c0)) ||
                                            (*(longlong *)
                                              ((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                             0x2a05877358342528)) ||
                                           (bVar2 = false,
                                           *(longlong *)
                                            ((longlong)puVar4 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                           -0x2d7431cce59f6330)) {
                                          bVar2 = true;
                                        }
                                        if (bVar2) {
                                          uVar8 = 0;
                                          do {
                                            puVar4 = (ushort *)
                                                     ((longlong)puVar3 +
                                                     (ulonglong)puVar3[uVar8 + 1]);
                                            uVar6 = (ulonglong)*puVar4;
                                            if (((*(longlong *)((longlong)puVar4 + uVar6 * 0x21 + 2)
                                                  != 0x50126c1f9cda329f) ||
                                                (*(longlong *)((longlong)puVar4 + uVar6 * 0x21 + 10)
                                                 != 0x67c3ec2339739bc0)) ||
                                               ((*(longlong *)
                                                  ((longlong)puVar4 + uVar6 * 0x21 + 0x12) !=
                                                 0x1c9c6ad4e763280d ||
                                                (bVar2 = false,
                                                *(longlong *)
                                                 ((longlong)puVar4 + uVar6 * 0x21 + 0x1a) !=
                                                0x57fed5d0bbfd7df4)))) {
                                              bVar2 = true;
                                            }
                                            if (!bVar2) {
LAB_ram_00023dc8:
                                              FUN_ram_0002df08(&local_18,0x4e7000000000000,0,
                                                               0x3e8000000000000);
                                              uVar7 = local_18;
                                              uVar10 = local_10;
                                              break;
                                            }
                                            if (uVar6 != 0) {
                                              plVar5 = (longlong *)((longlong)puVar4 + 3);
                                              uVar9 = 0;
                                              do {
                                                if (((*plVar5 != 0x50126c1f9cda329f) ||
                                                    (plVar5[1] != 0x67c3ec2339739bc0)) ||
                                                   ((plVar5[2] != 0x1c9c6ad4e763280d ||
                                                    (bVar2 = false, plVar5[3] != 0x57fed5d0bbfd7df4)
                                                    ))) {
                                                  bVar2 = true;
                                                }
                                                if (!bVar2) goto LAB_ram_00023dc8;
                                                plVar5 = (longlong *)((longlong)plVar5 + 0x21);
                                                uVar9 = uVar9 + 1;
                                              } while (uVar9 < uVar6);
                                            }
                                            uVar8 = uVar8 + 1;
                                            uVar7 = 0;
                                            uVar10 = 0;
                                          } while (uVar8 < *puVar3);
                                        }
                                        else {
                                          uVar7 = 0;
                                          uVar10 = 0;
                                        }
                                        goto LAB_ram_00023be8;
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    uVar7 = 0;
    uVar10 = 0;
  }
LAB_ram_00023be8:
  *(undefined8 *)(param_1 + 2) = uVar7;
  *(undefined8 *)(param_1 + 4) = uVar10;
  *param_1 = 0;
  return;
}

// Function: FUN_ram_00023e58
void FUN_ram_00023e58(undefined4 *param_1,undefined8 *param_2)

{
  ushort uVar1;
  bool bVar2;
  ushort *puVar3;
  longlong *plVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  ulonglong uVar7;
  undefined8 local_18;
  undefined8 local_10;
  
  puVar3 = (ushort *)*param_2;
  uVar7 = (ulonglong)*(ushort *)((longlong)puVar3 + param_2[1] + -2);
  uVar5 = 0;
  uVar6 = 0;
  if (uVar7 < *puVar3) {
    puVar3 = (ushort *)((longlong)puVar3 + (ulonglong)puVar3[uVar7 + 1]);
    uVar1 = *puVar3;
    plVar4 = (longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 2);
    if ((((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 2) != 0x4873bce2144ae3b5) ||
         (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) != -0x2911a2500a1ef197)) ||
        (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x60b8aa6da3403855)) ||
       (bVar2 = false,
       *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) != 0x103cc0bd736050b0)) {
      bVar2 = true;
    }
    if (bVar2) {
      if (((*plVar4 != -0x1e8395f2e7b51c4b) ||
          (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) != -0x51f325fec501496b))
         || ((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x98144e7e5ae3fa8
             || (bVar2 = false,
                *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                0x40ee2497930cf7ea)))) {
        bVar2 = true;
      }
      if (bVar2) {
        if (((*plVar4 != 0x6ec031f25bd57904) ||
            (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) != 0x71568ce6ec574ee))
           || ((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                0x518ef4a3deb2b1fd ||
               (bVar2 = false,
               *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
               -0x70ec43a95d324efe)))) {
          bVar2 = true;
        }
        if (bVar2) {
          if ((((*plVar4 != 0x715b8f7af9be1205) ||
               (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                -0x3fbd123929120c83)) ||
              (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
               -0x1178411a20edb01e)) ||
             (bVar2 = false,
             *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) != -0x4693a2c08ba113c1
             )) {
            bVar2 = true;
          }
          if (bVar2) {
            if (((*plVar4 != -0x3b66289859b23cf6) ||
                (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                 0x75b1926ae1365115)) ||
               ((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                 0x678ad2090231d088 ||
                (bVar2 = false,
                *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                -0x139993aed94b961d)))) {
              bVar2 = true;
            }
            if (bVar2) {
              if (((*plVar4 != 0x136d5ca2f1569155) ||
                  (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                   0x340d9a0ae6f72a4f)) ||
                 ((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                   -0x2a9d9b9ca96e3882 ||
                  (bVar2 = false,
                  *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                  0x698f3435f126add1)))) {
                bVar2 = true;
              }
              if (bVar2) {
                if ((((*plVar4 != -0x16a608d8d48b0286) ||
                     (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                      0x7a819dd33c7070c6)) ||
                    (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                     0x6dd2523bce0a93a0)) ||
                   (bVar2 = false,
                   *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                   -0x2c4478dc22ab5fac)) {
                  bVar2 = true;
                }
                if (bVar2) {
                  if (((*plVar4 != -0x44f118ed916356fa) ||
                      (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                       0x6e904b4c145c1835)) ||
                     ((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                       0x2a2f74470ab0ff18 ||
                      (bVar2 = false,
                      *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                      -0x2b367796f4eefba2)))) {
                    bVar2 = true;
                  }
                  if (bVar2) {
                    if (((*plVar4 != -0x4fc4eec7e6cb4135) ||
                        (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                         0x45acad558b7e296b)) ||
                       ((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                         0x59369b4a1734ee6f ||
                        (bVar2 = false,
                        *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                        0x42c79970523f5e6b)))) {
                      bVar2 = true;
                    }
                    if (bVar2) {
                      if ((((*plVar4 != -0x1d323195ffe246f3) ||
                           (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                            0x67889bcdcd17de84)) ||
                          (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                           0x5666dfd02b922d2b)) ||
                         (bVar2 = false,
                         *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                         0x548b03e01a423aa3)) {
                        bVar2 = true;
                      }
                      if (bVar2) {
                        if (((*plVar4 != -0x6c2c22b8abad132c) ||
                            (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                             0x1776bd19d4d98a5b)) ||
                           ((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                             0x6f034a62de39afcb ||
                            (bVar2 = false,
                            *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                            -0x5f19bd0c7dda6fc5)))) {
                          bVar2 = true;
                        }
                        if (bVar2) {
                          if (((*plVar4 != -0x1bb09aaaa3eacf65) ||
                              (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                               0x6493c705f351bd52)) ||
                             ((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                               0x262c1d3289763901 ||
                              (bVar2 = false,
                              *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                              0x5be22f238cb47253)))) {
                            bVar2 = true;
                          }
                          if (bVar2) {
                            if ((((*plVar4 != -0x7af703e2864bdf4) ||
                                 (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                                  0x2de7dd1cfc9a6d15)) ||
                                (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                 0x6bafec3babd968f6)) ||
                               (bVar2 = false,
                               *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                               -0x3726a59b99a8f2a9)) {
                              bVar2 = true;
                            }
                            if (bVar2) {
                              if (((*plVar4 != -0x372c55a8b3c334fc) ||
                                  (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                                   0x72e40dd1add9f2d5)) ||
                                 ((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12)
                                   != 0x42e6fdaa3eff7804 ||
                                  (bVar2 = false,
                                  *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a)
                                  != -0x3a991ec56a126c8d)))) {
                                bVar2 = true;
                              }
                              if (bVar2) {
                                if (((*plVar4 != -0xc5e8ffce1a16dfa) ||
                                    (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10)
                                     != -0x2070af22c1e0392a)) ||
                                   ((*(longlong *)
                                      ((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                     -0x4d27c110388d62ba ||
                                    (bVar2 = false,
                                    *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a)
                                    != -0x19ea30d62c1318f6)))) {
                                  bVar2 = true;
                                }
                                if (bVar2) {
                                  if ((((*plVar4 != -0x241f8dfce1a16dfa) ||
                                       (*(longlong *)
                                         ((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                                        0x77ca68769172b20b)) ||
                                      (*(longlong *)
                                        ((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                       0x533f7524d0ace446)) ||
                                     (bVar2 = false,
                                     *(longlong *)
                                      ((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                     -0x6567b74b076d7538)) {
                                    bVar2 = true;
                                  }
                                  if (bVar2) {
                                    if (((*plVar4 != 0x1be9073efd071895) ||
                                        (*(longlong *)
                                          ((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                                         0x103eb598830568a5)) ||
                                       ((*(longlong *)
                                          ((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                         -0x6f5cf633300ceda6 ||
                                        (bVar2 = false,
                                        *(longlong *)
                                         ((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                        -0x5ca0aa4a02280026)))) {
                                      bVar2 = true;
                                    }
                                    if (bVar2) {
                                      if (((*plVar4 != -0x2d9d51bf92ab29f4) ||
                                          (*(longlong *)
                                            ((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                                           0x45eefdfe7495b816)) ||
                                         ((*(longlong *)
                                            ((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                           0xbb5f49c7d946b85 ||
                                          (bVar2 = false,
                                          *(longlong *)
                                           ((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                          0x115c61a060bb5829)))) {
                                        bVar2 = true;
                                      }
                                      if (bVar2) {
                                        if ((((*plVar4 != 0x366a33d8ef74db2b) ||
                                             (*(longlong *)
                                               ((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) !=
                                              0x6819eac7d96353c0)) ||
                                            (*(longlong *)
                                              ((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                             0x2a05877358342528)) ||
                                           (bVar2 = false,
                                           *(longlong *)
                                            ((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                           -0x2d7431cce59f6330)) {
                                          bVar2 = true;
                                        }
                                        if (bVar2) {
                                          FUN_ram_0002df08(&local_18,0x190000000000000,0,
                                                           0x3e8000000000000);
                                          uVar5 = local_18;
                                          uVar6 = local_10;
                                        }
                                        else {
                                          uVar5 = 0;
                                          uVar6 = 0;
                                        }
                                        goto LAB_ram_00024ab8;
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    uVar5 = 0;
    uVar6 = 0;
  }
LAB_ram_00024ab8:
  *(undefined8 *)(param_1 + 2) = uVar5;
  *(undefined8 *)(param_1 + 4) = uVar6;
  *param_1 = 0;
  return;
}

// Function: FUN_ram_00024af8
void FUN_ram_00024af8(undefined4 *param_1,undefined8 *param_2)

{
  undefined2 uVar1;
  bool bVar2;
  longlong lVar3;
  byte bVar4;
  ulonglong uVar5;
  longlong *plVar6;
  int *piVar7;
  byte bVar8;
  undefined8 uVar9;
  ulonglong uVar10;
  undefined8 uVar11;
  char *pcVar12;
  longlong lVar13;
  ulonglong uVar14;
  char *pcVar15;
  ulonglong uVar16;
  ushort *puVar17;
  longlong *local_530;
  ulonglong local_528;
  undefined4 local_520;
  undefined1 local_51c;
  uint local_388;
  undefined4 uStack_384;
  int local_380;
  undefined4 uStack_37c;
  int local_378 [2];
  char acStack_370 [35];
  char acStack_34d [21];
  char local_338 [824];
  
  uVar11 = 0;
  puVar17 = (ushort *)*param_2;
  uVar5 = (ulonglong)*(ushort *)((longlong)puVar17 + param_2[1] + -2);
  uVar9 = 0;
  if (*puVar17 <= uVar5) goto LAB_ram_000256f8;
  puVar17 = (ushort *)((longlong)puVar17 + (ulonglong)puVar17[uVar5 + 1]);
  uVar5 = (ulonglong)*puVar17;
  plVar6 = (longlong *)((longlong)puVar17 + uVar5 * 0x21 + 2);
  if ((((*(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 2) != 0x6ec031f25bd57904) ||
       (*(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 10) != 0x71568ce6ec574ee)) ||
      (*(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 0x12) != 0x518ef4a3deb2b1fd)) ||
     (bVar2 = false, *(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 0x1a) != -0x70ec43a95d324efe))
  {
    bVar2 = true;
  }
  if (bVar2) {
    if (((*plVar6 != 0x4873bce2144ae3b5) ||
        (*(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 10) != -0x2911a2500a1ef197)) ||
       ((*(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 0x12) != 0x60b8aa6da3403855 ||
        (bVar2 = false, *(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 0x1a) != 0x103cc0bd736050b0
        )))) {
      bVar2 = true;
    }
    if (bVar2) {
      if (((*plVar6 != -0x44f118ed916356fa) ||
          (*(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 10) != 0x6e904b4c145c1835)) ||
         ((*(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 0x12) != 0x2a2f74470ab0ff18 ||
          (bVar2 = false,
          *(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 0x1a) != -0x2b367796f4eefba2)))) {
        bVar2 = true;
      }
      if (bVar2) {
        if ((((*plVar6 != 0x136d5ca2f1569155) ||
             (*(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 10) != 0x340d9a0ae6f72a4f)) ||
            (*(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 0x12) != -0x2a9d9b9ca96e3882)) ||
           (bVar2 = false,
           *(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 0x1a) != 0x698f3435f126add1)) {
          bVar2 = true;
        }
        uVar11 = 0;
        uVar9 = 0;
        if (bVar2) goto LAB_ram_000256f8;
      }
    }
  }
  if (((*plVar6 != 0x6ec031f25bd57904) ||
      (*(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 10) != 0x71568ce6ec574ee)) ||
     ((*(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 0x12) != 0x518ef4a3deb2b1fd ||
      (bVar2 = false, *(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 0x1a) != -0x70ec43a95d324efe)
      ))) {
    bVar2 = true;
  }
  if (bVar2) {
    if ((((*plVar6 != 0x4873bce2144ae3b5) ||
         (*(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 10) != -0x2911a2500a1ef197)) ||
        (*(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 0x12) != 0x60b8aa6da3403855)) ||
       (bVar2 = false, *(longlong *)((longlong)puVar17 + uVar5 * 0x21 + 0x1a) != 0x103cc0bd736050b0)
       ) {
      bVar2 = true;
    }
    uVar11 = 0;
    uVar9 = 0;
    if ((bVar2) ||
       (local_528 = (ulonglong)*(ushort *)((longlong)puVar17 + uVar5 * 0x21 + 0x22), local_528 < 0xc
       )) goto LAB_ram_000256f8;
    local_530 = (longlong *)((longlong)puVar17 + uVar5 * 0x21 + 0x24);
    lVar13 = *local_530;
    if (lVar13 != -0x77a4a414b3c0b4bf) {
      if (lVar13 != 0x65879cc54d18aca8) {
        if (lVar13 == -0x37788a1e6e613908) {
          uVar11 = 1;
          goto LAB_ram_00025510;
        }
        goto LAB_ram_000256f0;
      }
      uVar11 = 2;
    }
LAB_ram_00025510:
    local_520 = *(undefined4 *)((longlong)puVar17 + uVar5 * 0x21 + 0x2c);
    local_51c = (undefined1)uVar11;
    FUN_ram_00013658(&local_388,&local_530);
    uVar5 = (ulonglong)local_388;
    if (1 < uVar5) {
      pcVar12 = local_338;
      uVar16 = 0;
      uVar10 = 1;
      do {
        lVar13 = uVar16 * 0x38;
        pcVar15 = pcVar12;
        uVar16 = uVar10;
        do {
          if (((acStack_34d[lVar13] == pcVar15[0x23]) && (acStack_370[lVar13] == '\x01')) &&
             ((*pcVar15 == '\x01' && (acStack_370[lVar13 + 1] != pcVar15[1]))))
          goto LAB_ram_00025678;
          uVar16 = uVar16 + 1;
          pcVar15 = pcVar15 + 0x38;
        } while (uVar16 < uVar5);
        pcVar12 = pcVar12 + 0x38;
        uVar14 = uVar10 + 1;
        uVar16 = uVar10;
        uVar10 = uVar14;
      } while (uVar14 < uVar5);
    }
LAB_ram_000256f0:
    uVar11 = 0;
    uVar9 = 0;
  }
  else {
    uVar1 = *(undefined2 *)((longlong)puVar17 + uVar5 * 0x21 + 0x22);
    lVar13 = (longlong)puVar17 + uVar5 * 0x21 + 0x24;
    uVar16 = FUN_ram_000164d8(plVar6,lVar13);
    bVar4 = 0;
    uVar16 = uVar16 & 0xff;
    if (uVar16 < 7) {
      if (uVar16 < 4) {
        if (1 < uVar16) {
          uVar10 = 8;
          uVar14 = 7;
LAB_ram_000250c0:
          if ((uVar14 < uVar5) && (uVar10 < uVar5)) {
            if ((*(longlong *)((longlong)puVar17 + uVar14 * 0x21 + 3) !=
                 *(longlong *)((longlong)puVar17 + uVar10 * 0x21 + 3)) ||
               (((*(longlong *)((longlong)puVar17 + uVar14 * 0x21 + 0xb) !=
                  *(longlong *)((longlong)puVar17 + uVar10 * 0x21 + 0xb) ||
                 (*(longlong *)((longlong)puVar17 + uVar14 * 0x21 + 0x13) !=
                  *(longlong *)((longlong)puVar17 + uVar10 * 0x21 + 0x13))) ||
                (bVar2 = false,
                *(longlong *)((longlong)puVar17 + uVar14 * 0x21 + 0x1b) !=
                *(longlong *)((longlong)puVar17 + uVar10 * 0x21 + 0x1b))))) {
              bVar2 = true;
            }
            bVar4 = 1;
            if (bVar2) {
              bVar4 = 0;
            }
          }
        }
      }
      else if (1 < uVar16 - 4) {
LAB_ram_00025098:
        uVar10 = 4;
        uVar14 = 3;
        goto LAB_ram_000250c0;
      }
LAB_ram_00025188:
      bVar8 = 0;
      if ((uVar16 < 9) && ((0x153UL >> uVar16 & 1) != 0)) {
        lVar3 = uVar16 * 8;
        uVar16 = *(ulonglong *)(&DAT_ram_00033e60 + lVar3);
        if ((uVar5 <= uVar16) ||
           ((uVar10 = *(ulonglong *)(&DAT_ram_00033e18 + lVar3), uVar5 <= uVar10 ||
            (uVar14 = *(ulonglong *)(&DAT_ram_00033dd0 + lVar3), uVar5 <= uVar14))))
        goto LAB_ram_000253f0;
        if ((*(longlong *)((longlong)puVar17 + uVar10 * 0x21 + 3) !=
             *(longlong *)((longlong)puVar17 + uVar14 * 0x21 + 3)) ||
           (((*(longlong *)((longlong)puVar17 + uVar10 * 0x21 + 0xb) !=
              *(longlong *)((longlong)puVar17 + uVar14 * 0x21 + 0xb) ||
             (*(longlong *)((longlong)puVar17 + uVar10 * 0x21 + 0x13) !=
              *(longlong *)((longlong)puVar17 + uVar14 * 0x21 + 0x13))) ||
            (bVar2 = false,
            *(longlong *)((longlong)puVar17 + uVar10 * 0x21 + 0x1b) !=
            *(longlong *)((longlong)puVar17 + uVar14 * 0x21 + 0x1b))))) {
          bVar2 = true;
        }
        if (bVar2) {
          if (((*(longlong *)((longlong)puVar17 + uVar10 * 0x21 + 3) !=
                *(longlong *)((longlong)puVar17 + uVar16 * 0x21 + 3)) ||
              (*(longlong *)((longlong)puVar17 + uVar10 * 0x21 + 0xb) !=
               *(longlong *)((longlong)puVar17 + uVar16 * 0x21 + 0xb))) ||
             ((*(longlong *)((longlong)puVar17 + uVar10 * 0x21 + 0x13) !=
               *(longlong *)((longlong)puVar17 + uVar16 * 0x21 + 0x13) ||
              (bVar2 = false,
              *(longlong *)((longlong)puVar17 + uVar10 * 0x21 + 0x1b) !=
              *(longlong *)((longlong)puVar17 + uVar16 * 0x21 + 0x1b))))) {
            bVar2 = true;
          }
          if (bVar2) {
            if (((*(longlong *)((longlong)puVar17 + uVar14 * 0x21 + 3) !=
                  *(longlong *)((longlong)puVar17 + uVar16 * 0x21 + 3)) ||
                (*(longlong *)((longlong)puVar17 + uVar14 * 0x21 + 0xb) !=
                 *(longlong *)((longlong)puVar17 + uVar16 * 0x21 + 0xb))) ||
               ((*(longlong *)((longlong)puVar17 + uVar14 * 0x21 + 0x13) !=
                 *(longlong *)((longlong)puVar17 + uVar16 * 0x21 + 0x13) ||
                (bVar2 = false,
                *(longlong *)((longlong)puVar17 + uVar14 * 0x21 + 0x1b) !=
                *(longlong *)((longlong)puVar17 + uVar16 * 0x21 + 0x1b))))) {
              bVar2 = true;
            }
            bVar8 = 1;
            if (bVar2) {
              bVar8 = 0;
            }
            goto LAB_ram_000253f0;
          }
        }
      }
      else {
LAB_ram_000253f0:
        if (!(bool)(bVar4 | bVar8)) goto LAB_ram_00025408;
      }
    }
    else {
      if (uVar16 < 10) {
        if ((uVar16 != 7) && (uVar16 == 8)) goto LAB_ram_00025098;
        uVar10 = 7;
        uVar14 = 6;
        goto LAB_ram_000250c0;
      }
      if (uVar16 - 10 < 3) goto LAB_ram_00025188;
LAB_ram_00025408:
      FUN_ram_000149e0(&local_530,lVar13,uVar1);
      if ((char)local_530 == '\x02') goto LAB_ram_000256f0;
      FUN_ram_00031b28(&local_388,&local_530,0x1a8);
      lVar13 = FUN_ram_000159c0(&local_388);
      if ((char)local_388 == '\0') {
        if (local_380 != 0) {
          piVar7 = &local_380;
          goto LAB_ram_000254e0;
        }
      }
      else if (local_378[0] != 0) {
        piVar7 = local_378;
LAB_ram_000254e0:
        *piVar7 = 0;
      }
      uVar11 = 0;
      uVar9 = 0;
      if (lVar13 == 0) goto LAB_ram_000256f8;
    }
LAB_ram_00025678:
    FUN_ram_0002df08(&local_388,0x190000000000000,0,0x3e8000000000000);
    uVar9 = CONCAT44(uStack_37c,local_380);
    uVar11 = CONCAT44(uStack_384,local_388);
  }
LAB_ram_000256f8:
  *(undefined8 *)(param_1 + 2) = uVar11;
  *(undefined8 *)(param_1 + 4) = uVar9;
  *param_1 = 0;
  return;
}

// Function: FUN_ram_00025718
void FUN_ram_00025718(undefined4 *param_1,undefined8 *param_2)

{
  bool bVar1;
  ushort *puVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  undefined8 uVar8;
  ushort *puVar9;
  longlong *plVar10;
  undefined8 local_18;
  undefined8 local_10;
  
  puVar2 = (ushort *)*param_2;
  uVar5 = (ulonglong)*puVar2;
  if (uVar5 == 0) {
    uVar4 = 0;
    uVar8 = 0;
  }
  else {
    uVar6 = 0;
    do {
      puVar9 = (ushort *)((longlong)puVar2 + (ulonglong)puVar2[uVar6 + 1]);
      uVar7 = (ulonglong)*puVar9;
      if ((((*(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 2) != -0x50528c1a3b122cfa) ||
           (*(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 10) != 0x1e5b548b8973e5f3)) ||
          (*(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 0x12) != -0x77b12671fa722d18)) ||
         (bVar1 = false, *(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 0x1a) != 0xaea1b5732b3bce0)
         ) {
        bVar1 = true;
      }
      if (!bVar1) goto LAB_ram_00025b38;
      if (uVar7 != 0) {
        plVar10 = (longlong *)((longlong)puVar9 + 3);
        uVar3 = 0;
        do {
          if (((*plVar10 != -0x50528c1a3b122cfa) || (plVar10[1] != 0x1e5b548b8973e5f3)) ||
             ((plVar10[2] != -0x77b12671fa722d18 || (bVar1 = false, plVar10[3] != 0xaea1b5732b3bce0)
              ))) {
            bVar1 = true;
          }
          if (!bVar1) goto LAB_ram_00025b38;
          plVar10 = (longlong *)((longlong)plVar10 + 0x21);
          uVar3 = uVar3 + 1;
        } while (uVar3 < uVar7);
      }
      uVar6 = uVar6 + 1;
    } while (uVar6 < uVar5);
    uVar6 = 0;
    do {
      puVar9 = (ushort *)((longlong)puVar2 + (ulonglong)puVar2[uVar6 + 1]);
      uVar7 = (ulonglong)*puVar9;
      if (((*(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 2) != 0x50126c1f9cda329f) ||
          (*(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 10) != 0x67c3ec2339739bc0)) ||
         ((*(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 0x12) != 0x1c9c6ad4e763280d ||
          (bVar1 = false,
          *(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 0x1a) != 0x57fed5d0bbfd7df4)))) {
        bVar1 = true;
      }
      if (!bVar1) goto LAB_ram_00025b38;
      if (uVar7 != 0) {
        plVar10 = (longlong *)((longlong)puVar9 + 3);
        uVar3 = 0;
        do {
          if ((((*plVar10 == 0x50126c1f9cda329f) && (plVar10[1] == 0x67c3ec2339739bc0)) &&
              (plVar10[2] == 0x1c9c6ad4e763280d)) && (plVar10[3] == 0x57fed5d0bbfd7df4))
          goto LAB_ram_00025b38;
          plVar10 = (longlong *)((longlong)plVar10 + 0x21);
          uVar3 = uVar3 + 1;
        } while (uVar3 < uVar7);
      }
      uVar6 = uVar6 + 1;
      uVar4 = 0;
      uVar8 = 0;
    } while (uVar6 < uVar5);
  }
LAB_ram_00025ba8:
  *(undefined8 *)(param_1 + 2) = uVar4;
  *(undefined8 *)(param_1 + 4) = uVar8;
  *param_1 = 0;
  return;
LAB_ram_00025b38:
  FUN_ram_0002df08(&local_18,0x4e7000000000000,0,0x3e8000000000000);
  uVar4 = local_18;
  uVar8 = local_10;
  goto LAB_ram_00025ba8;
}

// Function: FUN_ram_00025bc8
void FUN_ram_00025bc8(undefined4 *param_1,undefined8 *param_2)

{
  bool bVar1;
  ushort *puVar2;
  ulonglong uVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  undefined8 uVar8;
  ushort *puVar9;
  longlong *plVar10;
  undefined8 local_18;
  undefined8 local_10;
  
  puVar2 = (ushort *)*param_2;
  uVar5 = (ulonglong)*puVar2;
  if (uVar5 == 0) {
    uVar4 = 0;
    uVar8 = 0;
  }
  else {
    uVar6 = 0;
    do {
      puVar9 = (ushort *)((longlong)puVar2 + (ulonglong)puVar2[uVar6 + 1]);
      uVar7 = (ulonglong)*puVar9;
      if ((((*(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 2) != -0x1f0bdccdd8a585e2) ||
           (*(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 10) != -0x1ad6fffefd421bd7)) ||
          (*(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 0x12) != 0x6df2d47a0d373d05)) ||
         (bVar1 = false,
         *(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 0x1a) != -0x541f646a124c2e4a)) {
        bVar1 = true;
      }
      if (!bVar1) goto LAB_ram_00025fe8;
      if (uVar7 != 0) {
        plVar10 = (longlong *)((longlong)puVar9 + 3);
        uVar3 = 0;
        do {
          if (((*plVar10 != -0x1f0bdccdd8a585e2) || (plVar10[1] != -0x1ad6fffefd421bd7)) ||
             ((plVar10[2] != 0x6df2d47a0d373d05 ||
              (bVar1 = false, plVar10[3] != -0x541f646a124c2e4a)))) {
            bVar1 = true;
          }
          if (!bVar1) goto LAB_ram_00025fe8;
          plVar10 = (longlong *)((longlong)plVar10 + 0x21);
          uVar3 = uVar3 + 1;
        } while (uVar3 < uVar7);
      }
      uVar6 = uVar6 + 1;
    } while (uVar6 < uVar5);
    uVar6 = 0;
    do {
      puVar9 = (ushort *)((longlong)puVar2 + (ulonglong)puVar2[uVar6 + 1]);
      uVar7 = (ulonglong)*puVar9;
      if (((*(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 2) != 0x58903f74c3b08902) ||
          (*(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 10) != -0x1cf01a0bacce9fee)) ||
         ((*(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 0x12) != 0x4392e2ef592aecdd ||
          (bVar1 = false,
          *(longlong *)((longlong)puVar9 + uVar7 * 0x21 + 0x1a) != 0x341ba97256f0ac88)))) {
        bVar1 = true;
      }
      if (!bVar1) goto LAB_ram_00025fe8;
      if (uVar7 != 0) {
        plVar10 = (longlong *)((longlong)puVar9 + 3);
        uVar3 = 0;
        do {
          if ((((*plVar10 == 0x58903f74c3b08902) && (plVar10[1] == -0x1cf01a0bacce9fee)) &&
              (plVar10[2] == 0x4392e2ef592aecdd)) && (plVar10[3] == 0x341ba97256f0ac88))
          goto LAB_ram_00025fe8;
          plVar10 = (longlong *)((longlong)plVar10 + 0x21);
          uVar3 = uVar3 + 1;
        } while (uVar3 < uVar7);
      }
      uVar6 = uVar6 + 1;
      uVar4 = 0;
      uVar8 = 0;
    } while (uVar6 < uVar5);
  }
LAB_ram_00026058:
  *(undefined8 *)(param_1 + 2) = uVar4;
  *(undefined8 *)(param_1 + 4) = uVar8;
  *param_1 = 0;
  return;
LAB_ram_00025fe8:
  FUN_ram_0002df08(&local_18,0x1f4000000000000,0,0x3e8000000000000);
  uVar4 = local_18;
  uVar8 = local_10;
  goto LAB_ram_00026058;
}

// Function: FUN_ram_00026078
void FUN_ram_00026078(undefined4 *param_1,undefined8 *param_2)

{
  ushort uVar1;
  bool bVar2;
  ushort *puVar3;
  undefined8 uVar4;
  ulonglong uVar5;
  undefined8 uVar6;
  undefined8 local_18;
  undefined8 local_10;
  
  puVar3 = (ushort *)*param_2;
  uVar5 = (ulonglong)*(ushort *)((longlong)puVar3 + param_2[1] + -2);
  uVar4 = 0;
  uVar6 = 0;
  if (uVar5 < *puVar3) {
    puVar3 = (ushort *)((longlong)puVar3 + (ulonglong)puVar3[uVar5 + 1]);
    uVar1 = *puVar3;
    if ((((*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 2) != 0x4873bce2144ae3b5) ||
         (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 10) != -0x2911a2500a1ef197)) ||
        (*(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x60b8aa6da3403855)) ||
       (bVar2 = false,
       *(longlong *)((longlong)puVar3 + (ulonglong)uVar1 * 0x21 + 0x1a) != 0x103cc0bd736050b0)) {
      bVar2 = true;
    }
    uVar4 = 0;
    if (!bVar2) {
      FUN_ram_0002df08(&local_18,0x3e8000000000000,0,0x3e8000000000000);
      uVar4 = local_18;
      uVar6 = local_10;
    }
  }
  *(undefined8 *)(param_1 + 2) = uVar4;
  *(undefined8 *)(param_1 + 4) = uVar6;
  *param_1 = 0;
  return;
}

// Function: FUN_ram_00026238
void FUN_ram_00026238(undefined4 *param_1,undefined8 *param_2,longlong *param_3)

{
  bool bVar1;
  longlong *plVar2;
  ushort *puVar3;
  bool bVar4;
  longlong **pplVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  longlong *plVar8;
  ulonglong uVar9;
  longlong *local_50;
  longlong *local_48;
  longlong local_40;
  longlong local_38;
  longlong local_30;
  longlong local_28;
  longlong local_20;
  longlong local_18;
  longlong local_10;
  longlong local_8;
  
  local_48 = &DAT_ram_00033620;
  local_50 = &DAT_ram_00033640;
  bVar4 = true;
  puVar3 = (ushort *)*param_2;
  plVar2 = &DAT_ram_00033c33;
  do {
    plVar8 = plVar2;
    if (bVar4) {
      if ((plVar2 == (longlong *)0x0) || (plVar2 == (longlong *)0x33cd3)) {
        pplVar5 = &local_48;
        plVar8 = (longlong *)0x0;
        if ((local_48 == (longlong *)0x0) ||
           (bVar4 = true, plVar2 = local_48, local_48 == &DAT_ram_00033640)) goto LAB_ram_00026368;
        goto LAB_ram_000263b0;
      }
      bVar4 = true;
      plVar8 = plVar2 + 4;
    }
    else {
LAB_ram_00026368:
      pplVar5 = &local_50;
      if ((local_50 == (longlong *)0x0) ||
         (bVar4 = false, plVar2 = local_50, local_50 == (longlong *)&DAT_ram_00033660)) {
        local_20 = 0;
        local_18 = 0;
LAB_ram_00026660:
        *(longlong *)(param_1 + 2) = local_20;
        *(longlong *)(param_1 + 4) = local_18;
        *param_1 = 0;
        return;
      }
LAB_ram_000263b0:
      *pplVar5 = plVar2 + 4;
    }
    local_28 = plVar2[3];
    local_30 = plVar2[2];
    local_38 = plVar2[1];
    local_40 = *plVar2;
    if ((((local_40 != *param_3) || (local_38 != param_3[1])) || (local_30 != param_3[2])) ||
       (bVar1 = false, local_28 != param_3[3])) {
      bVar1 = true;
    }
    plVar2 = plVar8;
    if ((bVar1) && ((ulonglong)*puVar3 != 0)) {
      uVar7 = 0;
      do {
        uVar9 = (ulonglong)*(ushort *)((longlong)puVar3 + (ulonglong)puVar3[uVar7 + 1]);
        if (uVar9 != 0) {
          plVar8 = (longlong *)((longlong)((longlong)puVar3 + (ulonglong)puVar3[uVar7 + 1]) + 3);
          uVar6 = 0;
          do {
            if (((*plVar8 != local_40) || (plVar8[1] != local_38)) ||
               ((plVar8[2] != local_30 || (bVar1 = false, plVar8[3] != local_28)))) {
              bVar1 = true;
            }
            if (!bVar1) {
              local_20 = local_40;
              local_18 = local_38;
              local_10 = local_30;
              local_8 = local_28;
              FUN_ram_0002df08(&local_20,0x4e7000000000000,0,0x3e8000000000000);
              goto LAB_ram_00026660;
            }
            plVar8 = (longlong *)((longlong)plVar8 + 0x21);
            uVar6 = uVar6 + 1;
          } while (uVar6 < uVar9);
        }
        uVar7 = uVar7 + 1;
      } while (uVar7 < *puVar3);
    }
  } while( true );
}

// Function: FUN_ram_000266a0
void FUN_ram_000266a0(undefined4 *param_1,undefined8 *param_2,ulonglong param_3)

{
  bool bVar1;
  ulonglong uVar2;
  ushort *puVar3;
  longlong lVar4;
  ulonglong uVar5;
  longlong *plVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  
  puVar3 = (ushort *)*param_2;
  uVar5 = (ulonglong)*puVar3;
  if (uVar5 != 0) {
    uVar2 = 0;
    do {
      uVar7 = (ulonglong)*(ushort *)((longlong)puVar3 + (ulonglong)puVar3[uVar2 + 1]);
      if (uVar7 != 0) {
        plVar6 = (longlong *)((longlong)((longlong)puVar3 + (ulonglong)puVar3[uVar2 + 1]) + 3);
        uVar8 = 0;
        do {
          if ((((*plVar6 != 0x3aca882143c3f10a) || (plVar6[1] != 0x1a95183aa1355163)) ||
              (plVar6[2] != 0x78a83d609f5abdce)) || (bVar1 = false, plVar6[3] != 0x77a3458a6218085e)
             ) {
            bVar1 = true;
          }
          if (!bVar1) goto LAB_ram_00026e38;
          plVar6 = (longlong *)((longlong)plVar6 + 0x21);
          uVar8 = uVar8 + 1;
        } while (uVar8 < uVar7);
      }
      uVar2 = uVar2 + 1;
    } while (uVar2 < uVar5);
    uVar2 = 0;
    do {
      uVar7 = (ulonglong)*(ushort *)((longlong)puVar3 + (ulonglong)puVar3[uVar2 + 1]);
      if (uVar7 != 0) {
        plVar6 = (longlong *)((longlong)((longlong)puVar3 + (ulonglong)puVar3[uVar2 + 1]) + 3);
        uVar8 = 0;
        do {
          if (((*plVar6 != 0x3aca882143c3f10a) || (plVar6[1] != -0x7672f262cd0bae9d)) ||
             ((plVar6[2] != 0x447a2fb3fc239b2f || (bVar1 = false, plVar6[3] != 0xb041ba6adf)))) {
            bVar1 = true;
          }
          if (!bVar1) goto LAB_ram_00026e38;
          plVar6 = (longlong *)((longlong)plVar6 + 0x21);
          uVar8 = uVar8 + 1;
        } while (uVar8 < uVar7);
      }
      uVar2 = uVar2 + 1;
    } while (uVar2 < uVar5);
    uVar2 = 0;
    do {
      uVar7 = (ulonglong)*(ushort *)((longlong)puVar3 + (ulonglong)puVar3[uVar2 + 1]);
      if (uVar7 != 0) {
        plVar6 = (longlong *)((longlong)((longlong)puVar3 + (ulonglong)puVar3[uVar2 + 1]) + 3);
        uVar8 = 0;
        do {
          if (((*plVar6 != 0x3aca882143c3f10a) || (plVar6[1] != 0x1a95183aa1355163)) ||
             ((plVar6[2] != 0x67ae2dace629bdce || (bVar1 = false, plVar6[3] != 0x3c000040d706dbff)))
             ) {
            bVar1 = true;
          }
          if (!bVar1) goto LAB_ram_00026e38;
          plVar6 = (longlong *)((longlong)plVar6 + 0x21);
          uVar8 = uVar8 + 1;
        } while (uVar8 < uVar7);
      }
      uVar2 = uVar2 + 1;
    } while (uVar2 < uVar5);
    uVar2 = 0;
    do {
      uVar7 = (ulonglong)*(ushort *)((longlong)puVar3 + (ulonglong)puVar3[uVar2 + 1]);
      if (uVar7 != 0) {
        plVar6 = (longlong *)((longlong)((longlong)puVar3 + (ulonglong)puVar3[uVar2 + 1]) + 3);
        uVar8 = 0;
        do {
          if ((((*plVar6 != -0x4794be73b93c0ef6) || (plVar6[1] != 0x35cd692c2af52327)) ||
              (plVar6[2] != -0x98a3353704f3ad0)) || (bVar1 = false, plVar6[3] != 0x3c00008063e199fd)
             ) {
            bVar1 = true;
          }
          if (!bVar1) goto LAB_ram_00026e38;
          plVar6 = (longlong *)((longlong)plVar6 + 0x21);
          uVar8 = uVar8 + 1;
        } while (uVar8 < uVar7);
      }
      uVar2 = uVar2 + 1;
    } while (uVar2 < uVar5);
    uVar2 = 0;
    do {
      uVar7 = (ulonglong)*(ushort *)((longlong)puVar3 + (ulonglong)puVar3[uVar2 + 1]);
      if (uVar7 != 0) {
        plVar6 = (longlong *)((longlong)((longlong)puVar3 + (ulonglong)puVar3[uVar2 + 1]) + 3);
        uVar8 = 0;
        do {
          if (((*plVar6 != 0x3aca882143c3f10a) || (plVar6[1] != 0x1a95183aa1355163)) ||
             ((plVar6[2] != -0x6c8fd15319d64232 || (bVar1 = false, plVar6[3] != -0x2d8f0e298cefc0d9)
              ))) {
            bVar1 = true;
          }
          if (!bVar1) goto LAB_ram_00026e38;
          plVar6 = (longlong *)((longlong)plVar6 + 0x21);
          uVar8 = uVar8 + 1;
        } while (uVar8 < uVar7);
      }
      uVar2 = uVar2 + 1;
    } while (uVar2 < uVar5);
  }
  uVar5 = param_3 >> 0x10;
  lVar4 = param_3 << 0x30;
LAB_ram_00026e70:
  *(longlong *)(param_1 + 2) = lVar4;
  *(ulonglong *)(param_1 + 4) = uVar5;
  *param_1 = 0;
  return;
LAB_ram_00026e38:
  lVar4 = 0;
  uVar5 = 0;
  goto LAB_ram_00026e70;
}

// Function: FUN_ram_00026e90
/* WARNING: Type propagation algorithm not settling */

void FUN_ram_00026e90(undefined8 *param_1,ulonglong param_2,longlong param_3,longlong param_4,
                     longlong param_5)

{
  ushort uVar1;
  undefined1 uVar2;
  ulonglong *puVar3;
  longlong lVar4;
  bool bVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  undefined8 uVar10;
  ulonglong uVar11;
  ulonglong *puVar12;
  ulonglong uVar13;
  ushort *puVar14;
  longlong *plVar15;
  ulonglong uVar16;
  undefined1 *puVar17;
  ulonglong uVar18;
  bool bVar19;
  longlong lVar20;
  ulonglong uVar21;
  ulonglong uVar22;
  undefined4 uVar23;
  longlong lVar24;
  longlong lVar25;
  ushort *puVar26;
  ulonglong uVar27;
  longlong lVar28;
  ulonglong uVar29;
  ulonglong *puVar30;
  ulonglong uVar31;
  ulonglong uVar32;
  longlong lVar33;
  ulonglong uVar34;
  ulonglong uVar35;
  ulonglong uVar36;
  longlong local_198;
  ulonglong local_180;
  ulonglong local_150;
  longlong local_f0;
  ulonglong local_e8;
  ulonglong local_e0;
  longlong local_d8;
  ulonglong local_d0;
  longlong local_c8;
  ulonglong local_c0;
  longlong local_b8;
  longlong local_b0;
  longlong local_a8;
  ulonglong local_a0;
  ulonglong local_98;
  ulonglong local_90;
  ulonglong local_88;
  ulonglong local_80;
  ulonglong local_78;
  ulonglong local_70;
  ulonglong local_68;
  longlong local_60;
  ulonglong local_58;
  ulonglong local_50;
  ulonglong local_48;
  ulonglong local_40;
  ulonglong local_38;
  ulonglong local_30;
  ulonglong local_28;
  int local_20;
  undefined4 uStack_1c;
  undefined4 local_18;
  uint uStack_14;
  ulonglong local_10;
  ulonglong local_8;
  
  puVar30 = *(ulonglong **)(param_5 + -0xff8);
  uVar6 = 0x6e9de2b30b19f9ea;
  if (puVar30[0xd7] < 4) {
    uVar6 = 0x6e9de2b30b19f1ea;
  }
  puVar30[0x48] = puVar30[0x48] ^ 0xb957ed15dc877426;
  puVar30[0x49] = puVar30[0x49] ^ 0x46a912eb23798bd9;
  puVar30[0x44] = puVar30[0x44] ^ 0xb957ed15dc877c26;
  puVar30[0x45] = puVar30[0x45] ^ 0x46a912eb237873d9;
  *puVar30 = *puVar30 ^ 0x69d190c683eda5d3;
  puVar30[1] = puVar30[1] ^ 0x962f6f387c135a2c;
  puVar30[2] = puVar30[2] ^ 0x962c6f3b7c105a2d;
  puVar30[3] = puVar30[3] ^ 0x962d6f3a7c115a2e;
  puVar30[0x4a] = puVar30[0x4a] ^ uVar6;
  puVar30[4] = puVar30[4] ^ 0x962a6f3d7c165a2f;
  puVar30[5] = puVar30[5] ^ 0x962b6f3c7c175a28;
  puVar30[6] = puVar30[6] ^ 0x96286f3f7c145a29;
  puVar30[7] = puVar30[7] ^ 0x96296f3e7c155a2a;
  puVar30[8] = puVar30[8] ^ 0x96266f317c1a5a2b;
  puVar30[9] = puVar30[9] ^ 0x96276f307c1b5a24;
  puVar30[10] = puVar30[10] ^ 0x96246f337c185a25;
  puVar30[0xb] = puVar30[0xb] ^ 0x96256f327c195a26;
  puVar30[0xc] = puVar30[0xc] ^ 0x96226f357c1e5a27;
  puVar30[0xd] = puVar30[0xd] ^ 0x96236f347c1f5a20;
  puVar30[0xe] = puVar30[0xe] ^ 0x96206f377c1c5a21;
  puVar30[0xf] = puVar30[0xf] ^ 0x96216f367c1d5a22;
  puVar30[0x10] = puVar30[0x10] ^ 0x963e6f297c025a23;
  puVar30[0x11] = puVar30[0x11] ^ 0x963f6f287c035a3c;
  FUN_ram_00002a50();
  puVar30[0x5a] = puVar30[0x5a] ^ 0xb82c93d08854ebff;
  puVar30[0x5b] = puVar30[0x5b] ^ 0x47d26c2e77aa1400;
  puVar30[0x5c] = puVar30[0x5c] ^ 0x47d16c2d77a91401;
  puVar30[0x5d] = puVar30[0x5d] ^ 0x47d06c2c77a81402;
  puVar30[0x5e] = puVar30[0x5e] ^ 0x47d76c2b77af1403;
  puVar30[0x5f] = puVar30[0x5f] ^ 0x47d66c2a77ae1404;
  puVar30[0x60] = puVar30[0x60] ^ 0x47d56c2977ad1405;
  puVar30[0x61] = puVar30[0x61] ^ 0x47d46c2877ac1406;
  puVar30[0x62] = puVar30[0x62] ^ 0x47db6c2777a31407;
  puVar30[99] = puVar30[99] ^ 0x47da6c2677a21408;
  puVar30[0x34] = puVar30[0x34] ^ 0xfb5ce87aae443c38;
  puVar30[0x35] = puVar30[0x35] ^ 0x4a2178451bac3c7;
  puVar30[0x36] = puVar30[0x36] ^ 0x4a1178751b9c3c6;
  puVar30[0x37] = puVar30[0x37] ^ 0x4a0178651b8c3c5;
  puVar30[0x30] = puVar30[0x30] ^ 0xfb5ce87aae443c38;
  puVar30[0x31] = puVar30[0x31] ^ 0x4a2178451bac3c7;
  puVar30[0x32] = puVar30[0x32] ^ 0x4a1178751b9c3c6;
  puVar30[0x33] = puVar30[0x33] ^ 0x4a0178651b8c3c5;
  uVar7 = *(ulonglong *)(param_5 + -0xfb8);
  local_150 = *(ulonglong *)(param_5 + -0xfc0);
  uVar6 = *(ulonglong *)(param_5 + -0xfb0);
  uVar8 = *(ulonglong *)(param_5 + -0xfc8);
  uVar9 = *(ulonglong *)(param_5 + -0xfd0);
  lVar24 = *(longlong *)(param_5 + -0xfd8);
  puVar26 = *(ushort **)(param_5 + -0xfe0);
  uVar10 = *(undefined8 *)(param_5 + -0xfe8);
  uVar11 = *(ulonglong *)(param_5 + -0xff0);
  lVar28 = *(longlong *)(param_5 + -0x1000);
  if (puVar30[0xd7] < 6) {
    local_180 = 0;
  }
  else {
    local_180 = puVar30[0x65] ^ 0x35f72d643d3464eb;
    puVar30[0x65] = local_180;
    puVar30[0x66] = puVar30[0x66] ^ 0x9578e14d1d0d9c4e;
  }
  puVar12 = puVar30 + 0x34;
  puVar3 = puVar30 + 0x30;
  if (lVar28 == 0) {
    puVar12 = puVar30 + 0x30;
    puVar3 = puVar30 + 0x34;
  }
  local_a0 = *puVar3;
  local_98 = puVar3[1];
  local_90 = puVar3[2];
  local_88 = puVar3[3];
  local_80 = *puVar12;
  local_20 = (int)local_80;
  uStack_1c = (undefined4)(local_80 >> 0x20);
  local_78 = puVar12[1];
  local_18 = (undefined4)local_78;
  uStack_14 = (uint)(local_78 >> 0x20);
  local_70 = puVar12[2];
  local_68 = puVar12[3];
  uVar31 = param_2 >> 0x10;
  lVar33 = param_2 << 0x30;
  local_60 = lVar33;
  local_58 = uVar31;
  local_40 = local_a0;
  local_38 = local_98;
  local_30 = local_90;
  local_28 = local_88;
  local_10 = local_70;
  local_8 = local_68;
  if (lVar28 != 0) {
    FUN_ram_00001708(&local_20,&local_60,puVar30[0x48],puVar30[0x49]);
    if (CONCAT44(uStack_1c,local_20) == 0) {
      *(undefined4 *)(param_1 + 1) = 0x41d;
      goto LAB_ram_00029070;
    }
    if ((longlong)param_2 < 0) {
      uVar23 = 0x421;
      goto LAB_ram_00027d30;
    }
    local_198 = -1;
    uVar29 = param_2;
    if (CONCAT44(uStack_14,local_18) == 0 && local_10 == 0) {
      param_2 = 0;
      goto LAB_ram_00027e38;
    }
    uVar23 = 0x424;
    if ((longlong)local_10 < 0) {
      if (0xffff7fffffffffff < local_10) {
        param_2 = (ulonglong)(uStack_14 >> 0x10) | local_10 << 0x10;
        goto LAB_ram_00027e38;
      }
    }
    else if ((local_10 < 0x1000000000000) &&
            (param_2 = (ulonglong)(uStack_14 >> 0x10) | local_10 << 0x10, -1 < (longlong)param_2))
    goto LAB_ram_00027e38;
    goto LAB_ram_00027d30;
  }
  uVar29 = puVar30[0x48];
  FUN_ram_00031e70(&local_c0,uVar31,0,uVar29);
  FUN_ram_00031e70(&local_d0,uVar29,0,lVar33,0);
  uVar34 = puVar30[0x49];
  FUN_ram_00031e70(&local_e0,uVar34,(longlong)uVar34 >> 0x3f,lVar33,lVar33 >> 0x3f);
  FUN_ram_00031e70(&local_b0,uVar34,(longlong)uVar34 >> 0x3f,uVar31,0);
  uVar27 = local_e0 + local_c0 + local_c8;
  lVar25 = local_d8 + (lVar33 >> 0x3f & uVar34) + (ulonglong)(uVar27 < local_e0);
  uVar35 = local_b8 + ((longlong)uVar29 >> 0x3f & uVar31) +
           (ulonglong)(local_c0 + local_c8 < local_c0);
  uVar29 = uVar35 + local_b0;
  uVar34 = uVar29 + lVar25;
  lVar25 = ((longlong)uVar35 >> 0x3f) + local_a8 + (ulonglong)(uVar29 < uVar35) + (lVar25 >> 0x3f) +
           (ulonglong)(uVar34 < uVar29);
  uVar29 = (longlong)(uVar34 * 0x10000) >> 0x3f;
  if ((uVar34 >> 0x30 | lVar25 * 0x10000) != uVar29 || lVar25 >> 0x30 != uVar29) {
    *(undefined4 *)(param_1 + 1) = 0x429;
    goto LAB_ram_00029070;
  }
  uVar29 = uVar34 * 0x10000 | uVar27 >> 0x30;
  uVar27 = uVar27 * 0x10000;
  if ((uVar27 == 0 && local_d0 >> 0x30 == 0) && uVar29 == 0) {
    uVar29 = 0;
joined_r0x00027d68:
    local_198 = 1;
    if (-1 < (longlong)param_2) {
LAB_ram_00027e38:
      uVar34 = param_4 - puVar30[0x4a];
      if (0 < (longlong)puVar30[0x4a] != (longlong)uVar34 < param_4) {
        *(undefined4 *)(param_1 + 1) = 0x436;
        goto LAB_ram_00029070;
      }
      lVar25 = param_2 * local_198;
      uVar35 = uVar34 + lVar25;
      bVar5 = (longlong)uVar35 < (longlong)uVar34;
      if (lVar25 < 0) {
        bVar5 = !bVar5;
      }
      if (bVar5) {
        *(undefined4 *)(param_1 + 1) = 0x439;
        goto LAB_ram_00029070;
      }
      lVar20 = param_3 - uVar29 * local_198;
      bVar5 = lVar20 < param_3;
      if (0 < (longlong)(uVar29 * local_198)) {
        bVar5 = !bVar5;
      }
      if (bVar5) {
        *(undefined4 *)(param_1 + 1) = 0x43d;
        goto LAB_ram_00029070;
      }
      bVar5 = lVar25 < 0;
      if (param_4 + lVar25 < param_4) {
        bVar5 = !bVar5;
      }
      if (bVar5) {
        *(undefined4 *)(param_1 + 1) = 0x440;
        goto LAB_ram_00029070;
      }
      uVar27 = 0;
      uVar32 = 0;
      if (1 < puVar30[0xd7]) {
        FUN_ram_00017df8(&local_20,param_4 + lVar25,lVar20,puVar30[0x48]);
        if (local_20 != 0) goto LAB_ram_00028190;
        uVar27 = CONCAT44(uStack_14,local_18);
        uVar32 = local_10;
      }
      lVar25 = (uVar34 ^ (longlong)uVar34 >> 0x3f) - ((longlong)uVar34 >> 0x3f);
      lVar20 = (uVar35 ^ (longlong)uVar35 >> 0x3f) - ((longlong)uVar35 >> 0x3f);
      bVar5 = lVar20 - lVar25 < lVar20;
      if (0 < lVar25) {
        bVar5 = !bVar5;
      }
      if (bVar5) {
        *(undefined4 *)(param_1 + 1) = 0x452;
        goto LAB_ram_00029070;
      }
      lVar4 = FUN_ram_00016ea8(lVar20 - lVar25,puVar30);
      if (lVar4 == 0) {
        *(undefined4 *)(param_1 + 1) = 0xbeef1;
        goto LAB_ram_00029070;
      }
      if ((longlong)uVar29 < 0) {
        uVar23 = 0x456;
LAB_ram_00029098:
        *(undefined4 *)(param_1 + 1) = uVar23;
        *param_1 = 1;
        return;
      }
      lVar4 = FUN_ram_00016f48(uVar29,puVar30 + 0x12);
      if (lVar4 == 0) {
        uVar23 = 0xbeef2;
        goto LAB_ram_00029098;
      }
      FUN_ram_00017f28(&local_20,lVar25,lVar20,param_2);
      uVar34 = local_10;
      if (local_20 != 0) {
LAB_ram_00028190:
        *(ulonglong *)((longlong)param_1 + 4) = CONCAT44(local_18,uStack_1c);
        *(undefined4 *)param_1 = 1;
        return;
      }
      lVar25 = CONCAT44(uStack_14,local_18);
      FUN_ram_0001b3c8(&local_20,param_2,puVar30[0x44],puVar30[0x45],lVar4);
      uVar35 = local_10;
      if (local_20 != 0) goto LAB_ram_00028190;
      lVar20 = CONCAT44(uStack_14,local_18);
      uVar36 = 0;
      uVar13 = (ulonglong)*(ushort *)((longlong)puVar26 + lVar24 + -2);
      if (uVar13 < *puVar26) {
        puVar14 = (ushort *)((longlong)puVar26 + (ulonglong)puVar26[uVar13 + 1]);
        uVar1 = *puVar14;
        plVar15 = (longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 2);
        if ((((*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 2) != 0x4873bce2144ae3b5)
             || (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
                 -0x2911a2500a1ef197)) ||
            (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12) != 0x60b8aa6da3403855
            )) || (bVar5 = false,
                  *(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                  0x103cc0bd736050b0)) {
          bVar5 = true;
        }
        uVar36 = 1;
        if (bVar5) {
          if (((*plVar15 != -0x1e8395f2e7b51c4b) ||
              (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
               -0x51f325fec501496b)) ||
             ((*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12) !=
               0x98144e7e5ae3fa8 ||
              (bVar5 = false,
              *(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
              0x40ee2497930cf7ea)))) {
            bVar5 = true;
          }
          if (bVar5) {
            if (((*plVar15 != 0x6ec031f25bd57904) ||
                (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
                 0x71568ce6ec574ee)) ||
               ((*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                 0x518ef4a3deb2b1fd ||
                (bVar5 = false,
                *(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                -0x70ec43a95d324efe)))) {
              bVar5 = true;
            }
            if (bVar5) {
              if ((((*plVar15 != 0x715b8f7af9be1205) ||
                   (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
                    -0x3fbd123929120c83)) ||
                  (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                   -0x1178411a20edb01e)) ||
                 (bVar5 = false,
                 *(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                 -0x4693a2c08ba113c1)) {
                bVar5 = true;
              }
              if (bVar5) {
                if (((*plVar15 != -0x3b66289859b23cf6) ||
                    (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
                     0x75b1926ae1365115)) ||
                   ((*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                     0x678ad2090231d088 ||
                    (bVar5 = false,
                    *(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                    -0x139993aed94b961d)))) {
                  bVar5 = true;
                }
                if (bVar5) {
                  if (((*plVar15 != 0x136d5ca2f1569155) ||
                      (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
                       0x340d9a0ae6f72a4f)) ||
                     ((*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                       -0x2a9d9b9ca96e3882 ||
                      (bVar5 = false,
                      *(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                      0x698f3435f126add1)))) {
                    bVar5 = true;
                  }
                  if (bVar5) {
                    if ((((*plVar15 != -0x16a608d8d48b0286) ||
                         (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
                          0x7a819dd33c7070c6)) ||
                        (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                         0x6dd2523bce0a93a0)) ||
                       (bVar5 = false,
                       *(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                       -0x2c4478dc22ab5fac)) {
                      bVar5 = true;
                    }
                    if (bVar5) {
                      if (((*plVar15 != -0x44f118ed916356fa) ||
                          (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
                           0x6e904b4c145c1835)) ||
                         ((*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                           0x2a2f74470ab0ff18 ||
                          (bVar5 = false,
                          *(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                          -0x2b367796f4eefba2)))) {
                        bVar5 = true;
                      }
                      if (bVar5) {
                        if (((*plVar15 != -0x4fc4eec7e6cb4135) ||
                            (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
                             0x45acad558b7e296b)) ||
                           ((*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                             0x59369b4a1734ee6f ||
                            (bVar5 = false,
                            *(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                            0x42c79970523f5e6b)))) {
                          bVar5 = true;
                        }
                        if (bVar5) {
                          if ((((*plVar15 != -0x1d323195ffe246f3) ||
                               (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
                                0x67889bcdcd17de84)) ||
                              (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                               0x5666dfd02b922d2b)) ||
                             (bVar5 = false,
                             *(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                             0x548b03e01a423aa3)) {
                            bVar5 = true;
                          }
                          if (bVar5) {
                            if (((*plVar15 != -0x6c2c22b8abad132c) ||
                                (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
                                 0x1776bd19d4d98a5b)) ||
                               ((*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12)
                                 != 0x6f034a62de39afcb ||
                                (bVar5 = false,
                                *(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                -0x5f19bd0c7dda6fc5)))) {
                              bVar5 = true;
                            }
                            if (bVar5) {
                              if (((*plVar15 != -0x1bb09aaaa3eacf65) ||
                                  (*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10)
                                   != 0x6493c705f351bd52)) ||
                                 ((*(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12)
                                   != 0x262c1d3289763901 ||
                                  (bVar5 = false,
                                  *(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a)
                                  != 0x5be22f238cb47253)))) {
                                bVar5 = true;
                              }
                              if (bVar5) {
                                if ((((*plVar15 != -0x7af703e2864bdf4) ||
                                     (*(longlong *)
                                       ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
                                      0x2de7dd1cfc9a6d15)) ||
                                    (*(longlong *)
                                      ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                     0x6bafec3babd968f6)) ||
                                   (bVar5 = false,
                                   *(longlong *)((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a)
                                   != -0x3726a59b99a8f2a9)) {
                                  bVar5 = true;
                                }
                                if (bVar5) {
                                  if (((*plVar15 != -0x372c55a8b3c334fc) ||
                                      (*(longlong *)
                                        ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
                                       0x72e40dd1add9f2d5)) ||
                                     ((*(longlong *)
                                        ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                       0x42e6fdaa3eff7804 ||
                                      (bVar5 = false,
                                      *(longlong *)
                                       ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                      -0x3a991ec56a126c8d)))) {
                                    bVar5 = true;
                                  }
                                  if (bVar5) {
                                    if (((*plVar15 != -0xc5e8ffce1a16dfa) ||
                                        (*(longlong *)
                                          ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
                                         -0x2070af22c1e0392a)) ||
                                       ((*(longlong *)
                                          ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                         -0x4d27c110388d62ba ||
                                        (bVar5 = false,
                                        *(longlong *)
                                         ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                        -0x19ea30d62c1318f6)))) {
                                      bVar5 = true;
                                    }
                                    if (bVar5) {
                                      if ((((*plVar15 != -0x241f8dfce1a16dfa) ||
                                           (*(longlong *)
                                             ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
                                            0x77ca68769172b20b)) ||
                                          (*(longlong *)
                                            ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12) !=
                                           0x533f7524d0ace446)) ||
                                         (bVar5 = false,
                                         *(longlong *)
                                          ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                         -0x6567b74b076d7538)) {
                                        bVar5 = true;
                                      }
                                      if (bVar5) {
                                        if (((*plVar15 != 0x1be9073efd071895) ||
                                            (*(longlong *)
                                              ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10) !=
                                             0x103eb598830568a5)) ||
                                           ((*(longlong *)
                                              ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12)
                                             != -0x6f5cf633300ceda6 ||
                                            (bVar5 = false,
                                            *(longlong *)
                                             ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a) !=
                                            -0x5ca0aa4a02280026)))) {
                                          bVar5 = true;
                                        }
                                        if (bVar5) {
                                          if (((*plVar15 != -0x2d9d51bf92ab29f4) ||
                                              (*(longlong *)
                                                ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10)
                                               != 0x45eefdfe7495b816)) ||
                                             ((*(longlong *)
                                                ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x12)
                                               != 0xbb5f49c7d946b85 ||
                                              (bVar5 = false,
                                              *(longlong *)
                                               ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a)
                                              != 0x115c61a060bb5829)))) {
                                            bVar5 = true;
                                          }
                                          if (bVar5) {
                                            if ((((*plVar15 != 0x366a33d8ef74db2b) ||
                                                 (*(longlong *)
                                                   ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 10
                                                   ) != 0x6819eac7d96353c0)) ||
                                                (*(longlong *)
                                                  ((longlong)puVar14 +
                                                  (ulonglong)uVar1 * 0x21 + 0x12) !=
                                                 0x2a05877358342528)) ||
                                               (bVar5 = false,
                                               *(longlong *)
                                                ((longlong)puVar14 + (ulonglong)uVar1 * 0x21 + 0x1a)
                                               != -0x2d7431cce59f6330)) {
                                              bVar5 = true;
                                            }
                                            if (bVar5) {
                                              uVar36 = 0;
                                            }
                                          }
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      uVar36 = uVar36 & uVar11;
      FUN_ram_0001cb08(&local_20,puVar26,lVar24,uVar10);
      uVar11 = local_10;
      if (local_20 != 0) goto LAB_ram_00028190;
      uVar13 = uVar27 + lVar25;
      uVar18 = uVar32 + uVar34 + (ulonglong)(uVar13 < uVar27);
      uVar23 = 0x47a;
      if (-1 < (longlong)((uVar32 ^ uVar34 ^ 0xffffffffffffffff) & (uVar32 ^ uVar18))) {
        uVar21 = uVar13 + lVar20;
        uVar13 = uVar18 + uVar35 + (ulonglong)(uVar21 < uVar13);
        uVar23 = 0x47c;
        if (-1 < (longlong)((uVar18 ^ uVar35 ^ 0xffffffffffffffff) & (uVar18 ^ uVar13))) {
          uVar18 = puVar30[0xd7];
          lVar24 = CONCAT44(uStack_14,local_18);
          if (((uVar36 ^ 0xffffffffffffffff) & (ulonglong)(uVar18 != 0)) != 0) {
            local_50 = uVar21 + ((puVar30[0x58] ^ 0xe80b) << 0x30);
            local_48 = uVar13 + (local_50 < uVar21);
            uVar22 = uVar13 ^ local_48;
            uVar16 = uVar13 ^ 0xffffffffffffffff;
            uVar21 = local_50;
            uVar13 = local_48;
            if ((longlong)(uVar16 & uVar22) < 0) {
              *(undefined4 *)(param_1 + 1) = 0x487;
              goto LAB_ram_00029070;
            }
          }
          if (uVar18 < 3) {
            uVar16 = uVar21;
            if (uVar18 != 0) {
              FUN_ram_000005a8(&local_20,(puVar30[0x59] ^ 0xd3198133b7c1776c) << 0x30,
                               (puVar30[0x59] ^ 0xd3198133b7c1776c) >> 0x10,0x4189374bc7,0);
              if (CONCAT44(uStack_1c,local_20) != 1) {
                *(undefined4 *)(param_1 + 1) = 0x4b1;
                goto LAB_ram_00029070;
              }
              FUN_ram_000005a8(&local_40,uVar21,uVar13,CONCAT44(uStack_14,local_18),local_10);
              local_50 = local_38;
              uVar16 = local_38;
              local_48 = local_30;
              uVar13 = local_30;
              if (local_40 == 0) {
                *(undefined4 *)(param_1 + 1) = 0x4b3;
                goto LAB_ram_00029070;
              }
            }
          }
          else {
            FUN_ram_000005a8(&local_20,uVar21,uVar13,(puVar30[0x59] ^ 0xd3198133b7c1776c) << 0x30,
                             (puVar30[0x59] ^ 0xd3198133b7c1776c) >> 0x10);
            if (CONCAT44(uStack_1c,local_20) == 0) {
              *(undefined4 *)(param_1 + 1) = 0x496;
              goto LAB_ram_00029070;
            }
            FUN_ram_000005a8(&local_40,CONCAT44(uStack_14,local_18),local_10,0x4189374bc7,0);
            if (local_40 == 0) {
              *(undefined4 *)(param_1 + 1) = 0x498;
              goto LAB_ram_00029070;
            }
            uVar2 = uVar21 < local_38;
            if (local_30 != uVar13) {
              uVar2 = (longlong)uVar13 < (longlong)local_30;
            }
            uVar16 = local_38;
            uVar18 = local_30;
            if (!(bool)uVar2) {
              uVar16 = uVar21;
              uVar18 = uVar13;
            }
            uVar13 = uVar18;
            if (uVar36 != 0) {
              FUN_ram_000005a8(&local_20,(puVar30[100] ^ 0x504156a22548f8dd) << 0x30,
                               (puVar30[100] ^ 0x504156a22548f8dd) >> 0x10,0x4189374bc7,0);
              if (CONCAT44(uStack_1c,local_20) != 1) {
                *(undefined4 *)(param_1 + 1) = 0x4a5;
                goto LAB_ram_00029070;
              }
              local_50 = uVar16 + CONCAT44(uStack_14,local_18);
              local_48 = uVar13 + local_10 + (ulonglong)(local_50 < uVar16);
              uVar36 = uVar13 ^ local_10;
              uVar18 = uVar13 ^ local_48;
              uVar16 = local_50;
              uVar13 = local_48;
              if ((longlong)((uVar36 ^ 0xffffffffffffffff) & uVar18) < 0) {
                *(undefined4 *)(param_1 + 1) = 0x4a8;
                goto LAB_ram_00029070;
              }
            }
          }
          if ((uVar8 & 1) == 0) {
            if ((longlong)uVar7 < 0) {
              uVar23 = 0x4bb;
              goto LAB_ram_00029098;
            }
            local_150 = uVar16 + local_150;
            uVar8 = uVar13 + uVar7 + (ulonglong)(local_150 < uVar16);
            uVar23 = 0x4bf;
            if ((longlong)((uVar13 ^ uVar7 ^ 0xffffffffffffffff) & (uVar13 ^ uVar8)) < 0)
            goto LAB_ram_00029098;
          }
          else {
            uVar23 = 0x4c7;
            lVar4 = uVar7 + (0x1000000000000 < local_150);
            if ((longlong)(uVar7 & -lVar4) < 0) goto LAB_ram_00029098;
            if (0 < lVar4) {
              uVar23 = 0x4c9;
              goto LAB_ram_00029098;
            }
            FUN_ram_000005a8(&local_20,uVar16,uVar13,0x1000000000000 - local_150);
            if (CONCAT44(uStack_1c,local_20) == 0) {
              *(undefined4 *)(param_1 + 1) = 0x4cd;
              goto LAB_ram_00029070;
            }
            local_150 = CONCAT44(uStack_14,local_18);
            uVar8 = local_10;
          }
          FUN_ram_00000908(&local_20,local_180 << 0x30,local_180 >> 0x10,0x3e8000000000000,0);
          if ((char)local_10 != '\0') {
            *(undefined4 *)(param_1 + 1) = 0x4d3;
            goto LAB_ram_00029070;
          }
          FUN_ram_000005a8(&local_20,CONCAT44(uStack_1c,local_20),CONCAT44(uStack_14,local_18),
                           uVar9 << 0x30,uVar9 >> 0x10);
          if (CONCAT44(uStack_1c,local_20) == 0) {
            *(undefined4 *)(param_1 + 1) = 0x4d6;
            goto LAB_ram_00029070;
          }
          FUN_ram_00000148(&local_f0,CONCAT44(uStack_14,local_18),local_10,0x3e8000000000000,0);
          uVar7 = local_150 + lVar24;
          uVar9 = uVar8 + uVar11 + (ulonglong)(uVar7 < local_150);
          uVar23 = 0x4dc;
          if (-1 < (longlong)((uVar8 ^ uVar11 ^ 0xffffffffffffffff) & (uVar8 ^ uVar9))) {
            uVar8 = uVar7 + local_f0;
            uVar7 = uVar9 + local_e8 + (ulonglong)(uVar8 < uVar7);
            uVar23 = 0x4de;
            if (-1 < (longlong)((uVar9 ^ local_e8 ^ 0xffffffffffffffff) & (uVar9 ^ uVar7))) {
              local_50 = uVar8 + (uVar6 << 0x30);
              uVar6 = uVar6 >> 0x10;
              local_48 = uVar7 + uVar6 + (ulonglong)(local_50 < uVar8);
              uVar23 = 0x4e0;
              if (-1 < (longlong)((uVar7 ^ uVar6 ^ 0xffffffffffffffff) & (uVar7 ^ local_48))) {
                FUN_ram_00001af8(&local_20,&local_50,0x2710000000000000,0);
                uVar6 = local_10;
                if (CONCAT44(uStack_1c,local_20) != 1) {
                  *(undefined4 *)(param_1 + 1) = 0x4e4;
                  goto LAB_ram_00029070;
                }
                bVar5 = true;
                bVar19 = true;
                if (lVar24 == 0) {
                  bVar19 = false;
                  if ((longlong)uVar11 < 1) goto LAB_ram_00029e40;
LAB_ram_00029d48:
                  if (uVar11 == 0) goto LAB_ram_00029e58;
LAB_ram_00029d58:
                  puVar17 = &DAT_ram_0003385e;
                  lVar24 = 4;
                  if (!bVar5) {
                    FUN_ram_0002a2b8();
                    return;
                  }
                }
                else {
                  if (0 < (longlong)uVar11) goto LAB_ram_00029d48;
LAB_ram_00029e40:
                  bVar5 = false;
                  if (uVar11 != 0) goto LAB_ram_00029d58;
LAB_ram_00029e58:
                  puVar17 = &DAT_ram_0003385e;
                  lVar24 = 4;
                  if (!bVar19) {
                    if (uVar29 < 1000000000000) {
                      puVar17 = &DAT_ram_0003386e;
                      if (((((uVar29 < 500000000000) &&
                            (puVar17 = &DAT_ram_00033862, uVar29 < 250000000000)) &&
                           ((puVar17 = &DAT_ram_0003386a, uVar29 < 100000000000 &&
                            ((puVar17 = &DAT_ram_0003387a, uVar29 < 25000000000 &&
                             (puVar17 = &DAT_ram_00033872, uVar29 < 10000000000)))))) &&
                          (puVar17 = &DAT_ram_0003387e, uVar29 < 1000000000)) &&
                         ((puVar17 = &DAT_ram_00033876, uVar29 < 100000000 &&
                          (puVar17 = &DAT_ram_00033866, uVar29 < 25000000)))) {
                        puVar17 = &DAT_ram_0003385e;
                      }
                    }
                    else {
                      uVar7 = uVar29 / 1000000000000 - 1;
                      if (uVar7 < 0x13) {
                        lVar24 = uVar7 * 4 + 4;
                        puVar17 = *(undefined1 **)(&DAT_ram_00034658 + uVar7 * 8);
                      }
                      else {
                        puVar17 = &DAT_ram_00033b46;
                        lVar24 = 0x50;
                      }
                    }
                  }
                }
                uVar7 = CONCAT44(uStack_14,local_18);
                FUN_ram_00029e90(puVar17,lVar24);
                uVar8 = 0x6e9de2b30b19f1ea;
                if (3 < puVar30[0xd7]) {
                  uVar8 = 0x6e9de2b30b19f9ea;
                }
                puVar30[0x44] = puVar30[0x44] ^ 0xb957ed15dc877c26;
                puVar30[0x45] = puVar30[0x45] ^ 0x46a912eb237873d9;
                puVar30[0x4a] = puVar30[0x4a] ^ uVar8;
                FUN_ram_00002448(puVar30);
                FUN_ram_00002a50(puVar30 + 0x12);
                FUN_ram_000028b8(puVar30 + 0x5a);
                puVar30[0x34] = puVar30[0x34] ^ 0xfb5ce87aae443c38;
                puVar30[0x35] = puVar30[0x35] ^ 0x4a2178451bac3c7;
                puVar30[0x36] = puVar30[0x36] ^ 0x4a1178751b9c3c6;
                puVar30[0x37] = puVar30[0x37] ^ 0x4a0178651b8c3c5;
                puVar30[0x30] = puVar30[0x30] ^ 0xfb5ce87aae443c38;
                puVar30[0x31] = puVar30[0x31] ^ 0x4a2178451bac3c7;
                puVar30[0x32] = puVar30[0x32] ^ 0x4a1178751b9c3c6;
                puVar30[0x33] = puVar30[0x33] ^ 0x4a0178651b8c3c5;
                if (5 < puVar30[0xd7]) {
                  puVar30[0x65] = puVar30[0x65] ^ 0x35f72d643d3464eb;
                  puVar30[0x66] = puVar30[0x66] ^ 0x9578e14d1d0d9c4e;
                }
                uVar8 = uVar7 + 0x1000000000000;
                uVar7 = uVar6 + (uVar8 < uVar7);
                if ((longlong)((uVar6 ^ 0xffffffffffffffff) & (uVar6 ^ uVar7)) < 0) {
                  *(undefined4 *)(param_1 + 1) = 0x516;
                  goto LAB_ram_00029070;
                }
                if (lVar28 != 0) {
                  FUN_ram_000005a8(&local_20,puVar30[0x48],puVar30[0x49],uVar8,uVar7);
                  if (CONCAT44(uStack_1c,local_20) == 0) {
                    *(undefined4 *)(param_1 + 1) = 0x53d;
                    goto LAB_ram_00029070;
                  }
                  FUN_ram_00001708(&local_20,&local_60,CONCAT44(uStack_14,local_18),local_10);
                  if (CONCAT44(uStack_1c,local_20) == 0) {
                    *(undefined4 *)(param_1 + 1) = 0x542;
                    goto LAB_ram_00029070;
                  }
                  uVar6 = CONCAT44(uStack_14,local_18);
                  puVar30[0x48] = puVar30[0x48] ^ 0xb957ed15dc877426;
                  puVar30[0x49] = puVar30[0x49] ^ 0x46a912eb23798bd9;
                  if (uVar6 != 0 || local_10 != 0) {
                    if (0xffffffffffff < local_10) {
                      *(undefined4 *)(param_1 + 1) = 0x54c;
                      goto LAB_ram_00029070;
                    }
LAB_ram_0002a6a8:
                    uVar6 = uVar6 >> 0x30 | local_10 << 0x10;
                    goto LAB_ram_0002a6c0;
                  }
LAB_ram_0002a698:
                  uVar6 = 0;
LAB_ram_0002a6c0:
                  param_1[5] = uVar27;
                  param_1[3] = lVar25;
                  param_1[1] = lVar20;
                  param_1[7] = uVar6;
                  param_1[6] = uVar32;
                  param_1[4] = uVar34;
                  param_1[2] = uVar35;
                  *(undefined4 *)param_1 = 0;
                  return;
                }
                FUN_ram_000005a8(&local_20,lVar33,uVar31,puVar30[0x48],puVar30[0x49]);
                uVar23 = 0x51f;
                if (CONCAT44(uStack_1c,local_20) != 0) {
                  local_40 = CONCAT44(uStack_14,local_18);
                  local_38 = local_10;
                  FUN_ram_00001708(&local_20,&local_40,uVar8,uVar7);
                  uVar23 = 0x524;
                  if (CONCAT44(uStack_1c,local_20) == 1) {
                    uVar6 = CONCAT44(uStack_14,local_18);
                    puVar30[0x48] = puVar30[0x48] ^ 0xb957ed15dc877426;
                    puVar30[0x49] = puVar30[0x49] ^ 0x46a912eb23798bd9;
                    if (uVar6 == 0 && local_10 == 0) goto LAB_ram_0002a698;
                    uVar23 = 0x52e;
                    if (local_10 < 0x1000000000000) goto LAB_ram_0002a6a8;
                  }
                }
                *(undefined4 *)(param_1 + 1) = uVar23;
                goto LAB_ram_00029070;
              }
            }
          }
        }
      }
LAB_ram_00027d30:
      *(undefined4 *)(param_1 + 1) = uVar23;
      goto LAB_ram_00029070;
    }
    uVar23 = 0x430;
  }
  else {
    uVar23 = 0x42f;
    if ((longlong)uVar29 < 0) {
      if (0xffff7fffffffffff < uVar29) {
        uVar29 = uVar27 >> 0x30 | uVar29 << 0x10;
        goto joined_r0x00027d68;
      }
    }
    else if ((uVar29 < 0x1000000000000) &&
            (uVar29 = uVar27 >> 0x30 | uVar29 << 0x10, -1 < (longlong)uVar29))
    goto joined_r0x00027d68;
  }
  *(undefined4 *)(param_1 + 1) = uVar23;
LAB_ram_00029070:
  *param_1 = 1;
  return;
}

// Function: FUN_ram_00029e90
void FUN_ram_00029e90(void)

{
  ulonglong uVar1;
  undefined4 uVar2;
  ulonglong unaff_R6;
  ulonglong unaff_R9;
  ulonglong uVar3;
  undefined8 local_1b8;
  undefined8 local_1b0;
  undefined8 local_1a8;
  undefined8 local_198;
  longlong local_138;
  undefined8 local_130;
  undefined8 local_128;
  undefined8 local_120;
  undefined8 local_118;
  undefined8 local_110;
  undefined8 *local_108;
  undefined8 local_100;
  longlong local_f8;
  undefined1 auStack_60 [32];
  ulonglong local_40;
  ulonglong local_38;
  longlong local_20;
  ulonglong local_18;
  ulonglong local_10;
  
  FUN_ram_00029e90();
  uVar1 = 0x6e9de2b30b19f1ea;
  if (3 < *(ulonglong *)(local_f8 + 0x6b8)) {
    uVar1 = 0x6e9de2b30b19f9ea;
  }
  *(ulonglong *)(local_f8 + 0x220) = *(ulonglong *)(local_f8 + 0x220) ^ 0xb957ed15dc877c26;
  *(ulonglong *)(local_f8 + 0x228) = *(ulonglong *)(local_f8 + 0x228) ^ 0x46a912eb237873d9;
  *(ulonglong *)(local_f8 + 0x250) = *(ulonglong *)(local_f8 + 0x250) ^ uVar1;
  FUN_ram_00002448(local_f8);
  FUN_ram_00002a50(local_130);
  FUN_ram_000028b8(local_198);
  *(ulonglong *)(local_f8 + 0x1a0) = *(ulonglong *)(local_f8 + 0x1a0) ^ 0xfb5ce87aae443c38;
  *(ulonglong *)(local_f8 + 0x1a8) = *(ulonglong *)(local_f8 + 0x1a8) ^ 0x4a2178451bac3c7;
  *(ulonglong *)(local_f8 + 0x1b0) = *(ulonglong *)(local_f8 + 0x1b0) ^ 0x4a1178751b9c3c6;
  *(ulonglong *)(local_f8 + 0x1b8) = *(ulonglong *)(local_f8 + 0x1b8) ^ 0x4a0178651b8c3c5;
  *(ulonglong *)(local_f8 + 0x180) = *(ulonglong *)(local_f8 + 0x180) ^ 0xfb5ce87aae443c38;
  *(ulonglong *)(local_f8 + 0x188) = *(ulonglong *)(local_f8 + 0x188) ^ 0x4a2178451bac3c7;
  *(ulonglong *)(local_f8 + 400) = *(ulonglong *)(local_f8 + 400) ^ 0x4a1178751b9c3c6;
  *(ulonglong *)(local_f8 + 0x198) = *(ulonglong *)(local_f8 + 0x198) ^ 0x4a0178651b8c3c5;
  if (5 < *(ulonglong *)(local_f8 + 0x6b8)) {
    *(ulonglong *)(local_f8 + 0x328) = *(ulonglong *)(local_f8 + 0x328) ^ 0x35f72d643d3464eb;
    *(ulonglong *)(local_f8 + 0x330) = *(ulonglong *)(local_f8 + 0x330) ^ 0x9578e14d1d0d9c4e;
  }
  uVar1 = unaff_R9 + 0x1000000000000;
  uVar3 = unaff_R6 + (uVar1 < unaff_R9);
  if ((longlong)((unaff_R6 ^ 0xffffffffffffffff) & (unaff_R6 ^ uVar3)) < 0) {
    *(undefined4 *)(local_108 + 1) = 0x516;
    goto LAB_ram_00029070;
  }
  if (local_138 != 0) {
    FUN_ram_000005a8(&local_20,*(undefined8 *)(local_f8 + 0x240),*(undefined8 *)(local_f8 + 0x248),
                     uVar1,uVar3);
    if (local_20 == 0) {
      *(undefined4 *)(local_108 + 1) = 0x53d;
      goto LAB_ram_00029070;
    }
    FUN_ram_00001708(&local_20,auStack_60,local_18,local_10);
    if (local_20 == 0) {
      *(undefined4 *)(local_108 + 1) = 0x542;
      goto LAB_ram_00029070;
    }
    *(ulonglong *)(local_f8 + 0x240) = *(ulonglong *)(local_f8 + 0x240) ^ 0xb957ed15dc877426;
    *(ulonglong *)(local_f8 + 0x248) = *(ulonglong *)(local_f8 + 0x248) ^ 0x46a912eb23798bd9;
    if (local_18 != 0 || local_10 != 0) {
      if (0xffffffffffff < local_10) {
        *(undefined4 *)(local_108 + 1) = 0x54c;
        goto LAB_ram_00029070;
      }
LAB_ram_0002a6a8:
      uVar1 = local_18 >> 0x30 | local_10 << 0x10;
      goto LAB_ram_0002a6c0;
    }
LAB_ram_0002a698:
    uVar1 = 0;
LAB_ram_0002a6c0:
    local_108[5] = local_120;
    local_108[3] = local_100;
    local_108[1] = local_1b8;
    local_108[7] = uVar1;
    local_108[6] = local_118;
    local_108[4] = local_1b0;
    local_108[2] = local_1a8;
    *(undefined4 *)local_108 = 0;
    return;
  }
  FUN_ram_000005a8(&local_20,local_110,local_128,*(undefined8 *)(local_f8 + 0x240),
                   *(undefined8 *)(local_f8 + 0x248));
  uVar2 = 0x51f;
  if (local_20 != 0) {
    local_40 = local_18;
    local_38 = local_10;
    FUN_ram_00001708(&local_20,&local_40,uVar1,uVar3);
    uVar2 = 0x524;
    if (local_20 == 1) {
      *(ulonglong *)(local_f8 + 0x240) = *(ulonglong *)(local_f8 + 0x240) ^ 0xb957ed15dc877426;
      *(ulonglong *)(local_f8 + 0x248) = *(ulonglong *)(local_f8 + 0x248) ^ 0x46a912eb23798bd9;
      if (local_18 == 0 && local_10 == 0) goto LAB_ram_0002a698;
      uVar2 = 0x52e;
      if (local_10 < 0x1000000000000) goto LAB_ram_0002a6a8;
    }
  }
  *(undefined4 *)(local_108 + 1) = uVar2;
LAB_ram_00029070:
  *local_108 = 1;
  return;
}

// Function: FUN_ram_0002a2b8
void FUN_ram_0002a2b8(undefined8 param_1,longlong param_2)

{
  ulonglong uVar1;
  undefined1 *puVar2;
  ulonglong uVar3;
  undefined4 uVar4;
  ulonglong uVar5;
  undefined8 uStack_1b8;
  undefined8 uStack_1b0;
  undefined8 uStack_1a8;
  ulonglong local_1a0;
  undefined8 uStack_198;
  longlong lStack_138;
  undefined8 uStack_130;
  undefined8 uStack_128;
  undefined8 uStack_120;
  undefined8 uStack_118;
  undefined8 uStack_110;
  undefined8 *puStack_108;
  undefined8 uStack_100;
  longlong lStack_f8;
  undefined1 auStack_60 [32];
  ulonglong uStack_40;
  ulonglong uStack_38;
  longlong lStack_20;
  ulonglong uStack_18;
  ulonglong uStack_10;
  
  uVar1 = uStack_10;
  uVar5 = uStack_18;
  if (local_1a0 < 1000000000000) {
    puVar2 = &DAT_ram_0003386e;
    if (((((local_1a0 < 500000000000) && (puVar2 = &DAT_ram_00033862, local_1a0 < 250000000000)) &&
         (puVar2 = &DAT_ram_0003386a, local_1a0 < 100000000000)) &&
        ((puVar2 = &DAT_ram_0003387a, local_1a0 < 25000000000 &&
         (puVar2 = &DAT_ram_00033872, local_1a0 < 10000000000)))) &&
       ((puVar2 = &DAT_ram_0003387e, local_1a0 < 1000000000 &&
        ((puVar2 = &DAT_ram_00033876, local_1a0 < 100000000 &&
         (puVar2 = &DAT_ram_00033866, local_1a0 < 25000000)))))) {
      puVar2 = &DAT_ram_0003385e;
    }
  }
  else {
    uVar3 = local_1a0 / 1000000000000 - 1;
    if (uVar3 < 0x13) {
      param_2 = uVar3 * 4 + 4;
      puVar2 = *(undefined1 **)(&DAT_ram_00034658 + uVar3 * 8);
    }
    else {
      puVar2 = &DAT_ram_00033b46;
      param_2 = 0x50;
    }
  }
  FUN_ram_00029e90(puVar2,param_2);
  uVar3 = 0x6e9de2b30b19f1ea;
  if (3 < *(ulonglong *)(lStack_f8 + 0x6b8)) {
    uVar3 = 0x6e9de2b30b19f9ea;
  }
  *(ulonglong *)(lStack_f8 + 0x220) = *(ulonglong *)(lStack_f8 + 0x220) ^ 0xb957ed15dc877c26;
  *(ulonglong *)(lStack_f8 + 0x228) = *(ulonglong *)(lStack_f8 + 0x228) ^ 0x46a912eb237873d9;
  *(ulonglong *)(lStack_f8 + 0x250) = *(ulonglong *)(lStack_f8 + 0x250) ^ uVar3;
  FUN_ram_00002448(lStack_f8);
  FUN_ram_00002a50(uStack_130);
  FUN_ram_000028b8(uStack_198);
  *(ulonglong *)(lStack_f8 + 0x1a0) = *(ulonglong *)(lStack_f8 + 0x1a0) ^ 0xfb5ce87aae443c38;
  *(ulonglong *)(lStack_f8 + 0x1a8) = *(ulonglong *)(lStack_f8 + 0x1a8) ^ 0x4a2178451bac3c7;
  *(ulonglong *)(lStack_f8 + 0x1b0) = *(ulonglong *)(lStack_f8 + 0x1b0) ^ 0x4a1178751b9c3c6;
  *(ulonglong *)(lStack_f8 + 0x1b8) = *(ulonglong *)(lStack_f8 + 0x1b8) ^ 0x4a0178651b8c3c5;
  *(ulonglong *)(lStack_f8 + 0x180) = *(ulonglong *)(lStack_f8 + 0x180) ^ 0xfb5ce87aae443c38;
  *(ulonglong *)(lStack_f8 + 0x188) = *(ulonglong *)(lStack_f8 + 0x188) ^ 0x4a2178451bac3c7;
  *(ulonglong *)(lStack_f8 + 400) = *(ulonglong *)(lStack_f8 + 400) ^ 0x4a1178751b9c3c6;
  *(ulonglong *)(lStack_f8 + 0x198) = *(ulonglong *)(lStack_f8 + 0x198) ^ 0x4a0178651b8c3c5;
  if (5 < *(ulonglong *)(lStack_f8 + 0x6b8)) {
    *(ulonglong *)(lStack_f8 + 0x328) = *(ulonglong *)(lStack_f8 + 0x328) ^ 0x35f72d643d3464eb;
    *(ulonglong *)(lStack_f8 + 0x330) = *(ulonglong *)(lStack_f8 + 0x330) ^ 0x9578e14d1d0d9c4e;
  }
  uVar3 = uVar5 + 0x1000000000000;
  uVar5 = uVar1 + (uVar3 < uVar5);
  if ((longlong)((uVar1 ^ 0xffffffffffffffff) & (uVar1 ^ uVar5)) < 0) {
    *(undefined4 *)(puStack_108 + 1) = 0x516;
    goto LAB_ram_00029070;
  }
  if (lStack_138 != 0) {
    FUN_ram_000005a8(&lStack_20,*(undefined8 *)(lStack_f8 + 0x240),
                     *(undefined8 *)(lStack_f8 + 0x248),uVar3,uVar5);
    if (lStack_20 == 0) {
      *(undefined4 *)(puStack_108 + 1) = 0x53d;
      goto LAB_ram_00029070;
    }
    FUN_ram_00001708(&lStack_20,auStack_60,uStack_18,uStack_10);
    if (lStack_20 == 0) {
      *(undefined4 *)(puStack_108 + 1) = 0x542;
      goto LAB_ram_00029070;
    }
    *(ulonglong *)(lStack_f8 + 0x240) = *(ulonglong *)(lStack_f8 + 0x240) ^ 0xb957ed15dc877426;
    *(ulonglong *)(lStack_f8 + 0x248) = *(ulonglong *)(lStack_f8 + 0x248) ^ 0x46a912eb23798bd9;
    if (uStack_18 != 0 || uStack_10 != 0) {
      if (0xffffffffffff < uStack_10) {
        *(undefined4 *)(puStack_108 + 1) = 0x54c;
        goto LAB_ram_00029070;
      }
LAB_ram_0002a6a8:
      uVar5 = uStack_18 >> 0x30 | uStack_10 << 0x10;
      goto LAB_ram_0002a6c0;
    }
LAB_ram_0002a698:
    uVar5 = 0;
LAB_ram_0002a6c0:
    puStack_108[5] = uStack_120;
    puStack_108[3] = uStack_100;
    puStack_108[1] = uStack_1b8;
    puStack_108[7] = uVar5;
    puStack_108[6] = uStack_118;
    puStack_108[4] = uStack_1b0;
    puStack_108[2] = uStack_1a8;
    *(undefined4 *)puStack_108 = 0;
    return;
  }
  FUN_ram_000005a8(&lStack_20,uStack_110,uStack_128,*(undefined8 *)(lStack_f8 + 0x240),
                   *(undefined8 *)(lStack_f8 + 0x248));
  uVar4 = 0x51f;
  if (lStack_20 != 0) {
    uStack_40 = uStack_18;
    uStack_38 = uStack_10;
    FUN_ram_00001708(&lStack_20,&uStack_40,uVar3,uVar5);
    uVar4 = 0x524;
    if (lStack_20 == 1) {
      *(ulonglong *)(lStack_f8 + 0x240) = *(ulonglong *)(lStack_f8 + 0x240) ^ 0xb957ed15dc877426;
      *(ulonglong *)(lStack_f8 + 0x248) = *(ulonglong *)(lStack_f8 + 0x248) ^ 0x46a912eb23798bd9;
      if (uStack_18 == 0 && uStack_10 == 0) goto LAB_ram_0002a698;
      uVar4 = 0x52e;
      if (uStack_10 < 0x1000000000000) goto LAB_ram_0002a6a8;
    }
  }
  *(undefined4 *)(puStack_108 + 1) = uVar4;
LAB_ram_00029070:
  *puStack_108 = 1;
  return;
}

// Function: FUN_ram_0002a740
void FUN_ram_0002a740(undefined8 *param_1,undefined8 param_2,undefined8 param_3)

{
  longlong lVar1;
  undefined8 local_68;
  undefined8 local_60;
  undefined *local_58;
  undefined8 local_50;
  undefined8 local_48;
  undefined8 local_40;
  undefined1 local_31;
  undefined *puStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  undefined8 uStack_18;
  undefined8 uStack_10;
  
  local_58 = &DAT_ram_00033580;
  local_40 = 0x20;
  local_50 = 0x20;
  local_60 = 0x20;
  local_31 = 0xff;
  local_68 = param_2;
  local_48 = param_3;
  lVar1 = FUN_ram_0002a7d8(&local_68,3,&DAT_ram_00033540,&puStack_30,&local_31);
  if (lVar1 == 0) {
    param_1[3] = uStack_18;
    param_1[2] = uStack_20;
    param_1[1] = uStack_28;
    *param_1 = puStack_30;
    return;
  }
  puStack_30 = &DAT_ram_000341e8;
  uStack_10 = 0;
  uStack_28 = 1;
  uStack_18 = 0;
  uStack_20 = 8;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&puStack_30,&DAT_ram_000341f8);
}

// Function: FUN_ram_0002a7d8
void FUN_ram_0002a7d8(void)

{
  longlong lVar1;
  undefined8 *unaff_R6;
  undefined *local_30;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_18;
  undefined8 local_10;
  
  lVar1 = FUN_ram_0002a7d8();
  if (lVar1 == 0) {
    unaff_R6[3] = local_18;
    unaff_R6[2] = local_20;
    unaff_R6[1] = local_28;
    *unaff_R6 = local_30;
    return;
  }
  local_30 = &DAT_ram_000341e8;
  local_10 = 0;
  local_28 = 1;
  local_18 = 0;
  local_20 = 8;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&local_30,&DAT_ram_000341f8);
}

// Function: FUN_ram_0002a890
/* WARNING: Type propagation algorithm not settling */

void FUN_ram_0002a890(ulonglong *param_1,ulonglong *param_2,ulonglong *param_3)

{
  undefined *puVar1;
  ulonglong uVar2;
  bool bVar3;
  ulonglong uVar4;
  undefined8 uVar5;
  undefined *puVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  undefined *puVar9;
  bool bVar10;
  undefined **ppuVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  ulonglong *puVar20;
  ulonglong uVar21;
  undefined *puVar22;
  longlong lVar23;
  ulonglong uVar24;
  longlong *plVar25;
  ulonglong uVar26;
  undefined *puVar27;
  undefined **local_190;
  longlong local_188 [2];
  ulonglong local_178;
  undefined8 local_170;
  longlong local_168 [2];
  ulonglong local_158;
  undefined8 local_150;
  longlong local_148 [2];
  ulonglong local_138;
  undefined8 local_130;
  longlong local_128;
  longlong local_120;
  longlong local_118;
  longlong local_110;
  longlong local_108;
  longlong local_100;
  undefined *local_f8;
  undefined *local_f0;
  undefined *local_e8;
  undefined *local_e0;
  longlong local_d8 [2];
  ulonglong local_c8;
  undefined8 local_c0;
  undefined *local_b8 [2];
  ulonglong local_a8;
  ulonglong local_a0;
  ulonglong local_98;
  undefined *local_88 [2];
  ulonglong local_78;
  ulonglong local_70;
  undefined *local_68 [2];
  ulonglong local_58;
  ulonglong local_50;
  ulonglong local_48;
  ulonglong local_40 [8];
  
  lVar23 = -0x40;
  uVar19 = param_2[3];
  uVar15 = uVar19;
  if (uVar19 == 0) {
    lVar23 = -0x80;
    uVar15 = param_2[2];
    if (uVar15 != 0) goto LAB_ram_0002a8e8;
    lVar23 = -0xc0;
    uVar15 = param_2[1];
    if (uVar15 != 0) goto LAB_ram_0002a8e8;
    uVar16 = *param_2;
    uVar15 = 0x40;
    if (uVar16 != 0) {
      uVar16 = uVar16 | uVar16 >> 1;
      uVar16 = uVar16 | uVar16 >> 2;
      uVar16 = uVar16 | uVar16 >> 4;
      uVar16 = uVar16 | uVar16 >> 8;
      uVar16 = uVar16 | uVar16 >> 0x10;
      uVar15 = (uVar16 | uVar16 >> 0x20) ^ 0xffffffffffffffff;
      uVar15 = uVar15 - (uVar15 >> 1 & 0x5555555555555555);
      uVar15 = (uVar15 & 0x3333333333333333) + (uVar15 >> 2 & 0x3333333333333333);
      uVar15 = (uVar15 + (uVar15 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38;
    }
    uVar15 = 0x40 - uVar15;
    uVar16 = param_3[3];
  }
  else {
LAB_ram_0002a8e8:
    uVar15 = uVar15 | uVar15 >> 1;
    uVar15 = uVar15 | uVar15 >> 2;
    uVar15 = uVar15 | uVar15 >> 4;
    uVar15 = uVar15 | uVar15 >> 8;
    uVar15 = uVar15 | uVar15 >> 0x10;
    uVar15 = (uVar15 | uVar15 >> 0x20) ^ 0xffffffffffffffff;
    uVar15 = uVar15 - (uVar15 >> 1 & 0x5555555555555555);
    uVar15 = (uVar15 & 0x3333333333333333) + (uVar15 >> 2 & 0x3333333333333333);
    uVar15 = (lVar23 - ((uVar15 + (uVar15 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38)) +
             0x140;
    uVar16 = param_3[3];
  }
  lVar23 = -0x40;
  if (uVar16 == 0) {
    lVar23 = -0x80;
    uVar16 = param_3[2];
    if (uVar16 == 0) {
      lVar23 = -0xc0;
      uVar16 = param_3[1];
      if (uVar16 == 0) {
        uVar16 = *param_3;
        if (uVar16 == 0) {
LAB_ram_0002c290:
          local_b8[0] = &DAT_ram_000340d0;
          local_98 = 0;
          local_b8[1] = (undefined *)0x1;
          local_a0 = 0;
          local_a8 = 8;
                    /* WARNING: Subroutine does not return */
          FUN_ram_0002fba8(local_b8,&DAT_ram_00034210);
        }
        uVar16 = uVar16 | uVar16 >> 1;
        uVar16 = uVar16 | uVar16 >> 2;
        uVar16 = uVar16 | uVar16 >> 4;
        uVar16 = uVar16 | uVar16 >> 8;
        uVar16 = uVar16 | uVar16 >> 0x10;
        uVar16 = (uVar16 | uVar16 >> 0x20) ^ 0xffffffffffffffff;
        uVar16 = uVar16 - (uVar16 >> 1 & 0x5555555555555555);
        uVar16 = (uVar16 & 0x3333333333333333) + (uVar16 >> 2 & 0x3333333333333333);
        uVar16 = 0x40 - ((uVar16 + (uVar16 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38);
        goto joined_r0x0002ac28;
      }
    }
  }
  uVar16 = uVar16 | uVar16 >> 1;
  uVar16 = uVar16 | uVar16 >> 2;
  uVar16 = uVar16 | uVar16 >> 4;
  uVar16 = uVar16 | uVar16 >> 8;
  uVar16 = uVar16 | uVar16 >> 0x10;
  uVar16 = (uVar16 | uVar16 >> 0x20) ^ 0xffffffffffffffff;
  uVar16 = uVar16 - (uVar16 >> 1 & 0x5555555555555555);
  uVar16 = (uVar16 & 0x3333333333333333) + (uVar16 >> 2 & 0x3333333333333333);
  uVar16 = (lVar23 - ((uVar16 + (uVar16 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38)) +
           0x140;
joined_r0x0002ac28:
  if (uVar15 < uVar16) {
    param_1[3] = 0;
    param_1[2] = 0;
    param_1[1] = 0;
    *param_1 = 0;
    param_1[4] = *param_2;
    param_1[5] = param_2[1];
    param_1[6] = param_2[2];
    param_1[7] = param_2[3];
    return;
  }
  if (uVar16 < 0x41) {
    uVar15 = *param_3;
    if (uVar15 == 0) {
      puVar9 = &DAT_ram_00034210;
      FUN_ram_00031548();
      FUN_ram_0002c398(**(undefined8 **)(puVar9 + 8),(*(undefined8 **)(puVar9 + 8))[1]);
      FUN_ram_0002c3b8("** PANICKED **",0xe);
      return;
    }
    uVar26 = param_2[2];
    uVar24 = param_2[1];
    uVar16 = *param_2;
    FUN_ram_000334a8(&local_138,uVar26,uVar19 - (uVar19 / uVar15) * uVar15,uVar15,0);
    FUN_ram_00031e70(local_148,local_138,local_130,uVar15,0);
    FUN_ram_000334a8(&local_158,uVar24,uVar26 - local_148[0],uVar15,0);
    FUN_ram_00031e70(local_168,local_158,local_150,uVar15,0);
    FUN_ram_000334a8(&local_178,uVar16,uVar24 - local_168[0],uVar15,0);
    FUN_ram_00031e70(local_188,local_178,local_170,uVar15,0);
    param_1[3] = uVar19 / uVar15;
    param_1[2] = local_138;
    param_1[1] = local_158;
    *param_1 = local_178;
    param_1[4] = uVar16 - local_188[0];
    param_1[5] = 0;
    param_1[6] = 0;
    param_1[7] = 0;
  }
  else {
    local_70 = param_3[3];
    local_78 = param_3[2];
    local_88[1] = (undefined *)param_3[1];
    local_88[0] = (undefined *)*param_3;
    uVar24 = uVar16 - 1 >> 6;
    uVar16 = (ulonglong)local_b8[uVar24 + 6];
    if (uVar16 == 0) {
      uVar26 = 0x40;
    }
    else {
      uVar26 = uVar16 | uVar16 >> 1;
      uVar26 = uVar26 | uVar26 >> 2;
      uVar26 = uVar26 | uVar26 >> 4;
      uVar26 = uVar26 | uVar26 >> 8;
      uVar26 = uVar26 | uVar26 >> 0x10;
      uVar26 = (uVar26 | uVar26 >> 0x20) ^ 0xffffffffffffffff;
      uVar26 = uVar26 - (uVar26 >> 1 & 0x5555555555555555);
      uVar26 = (uVar26 & 0x3333333333333333) + (uVar26 >> 2 & 0x3333333333333333);
      uVar26 = (uVar26 + (uVar26 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38;
    }
    uVar12 = (ulonglong)(uVar16 == 0);
    local_a0 = 0;
    local_a8 = 0;
    local_b8[1] = (undefined *)0x0;
    local_b8[0] = (undefined *)0x0;
    uVar4 = uVar26 & 0x3f;
    local_b8[uVar12] = (undefined *)((longlong)local_88[0] << uVar4);
    puVar9 = local_b8[0];
    lVar23 = 0x10;
    if (uVar16 != 0) {
      lVar23 = 8;
    }
    plVar25 = (longlong *)((longlong)local_b8 + lVar23);
    *plVar25 = (longlong)local_88[1] << uVar4;
    local_b8[uVar12 + 2] = (undefined *)(local_78 << uVar4);
    if (uVar16 != 0) {
      local_a0 = local_70 << uVar4;
    }
    if (uVar4 != 0) {
      uVar13 = -uVar26 & 0x3f;
      *plVar25 = *plVar25 + ((ulonglong)local_88[0] >> uVar13);
      local_b8[uVar12 + 2] =
           (undefined *)((longlong)local_b8[uVar12 + 2] + ((ulonglong)local_88[1] >> uVar13));
      if (uVar16 != 0) {
        local_a0 = local_a0 + (local_78 >> uVar13);
      }
    }
    uVar12 = local_a0;
    uVar16 = local_a8;
    puVar1 = local_b8[1];
    uVar13 = uVar15 - 1 >> 6;
    local_70 = local_a0;
    local_78 = local_a8;
    local_88[1] = local_b8[1];
    local_88[0] = local_b8[0];
    uVar17 = 0x40 - uVar26 >> 6;
    puVar20 = param_2 + uVar17;
    uVar14 = 0x40 - uVar26 & 0x3f;
    uVar15 = puVar20[2];
    local_50 = uVar15 >> uVar14;
    local_68[1] = (undefined *)(*puVar20 >> uVar14);
    uVar21 = puVar20[1];
    local_58 = uVar21 >> uVar14;
    if (uVar26 == 0) {
      local_48 = 0;
      if (uVar14 != 0) {
        local_58 = uVar15 + local_58;
        local_68[1] = local_68[1] + uVar21;
      }
    }
    else {
      local_48 = param_2[uVar17 + 3] >> uVar14;
      if (uVar14 != 0) {
        local_50 = (uVar19 << uVar4) + local_50;
        local_58 = (uVar15 << uVar4) + local_58;
        local_68[1] = local_68[1] + (uVar21 << uVar4);
      }
    }
    uVar19 = uVar13 - uVar24;
    uVar14 = uVar24 + 1;
    local_68[0] = (undefined *)(*param_2 << uVar4);
    local_190 = local_68 + (uVar13 - uVar24);
    uVar5 = local_b8[uVar24 + 5];
    local_40[3] = 0;
    local_40[2] = 0;
    local_40[1] = 0;
    local_40[0] = 0;
    puVar6 = local_b8[uVar24 + 6];
    uVar15 = uVar19;
    do {
      uVar13 = uVar15 + uVar14;
      if (4 < uVar13) {
LAB_ram_0002c2f0:
                    /* WARNING: Subroutine does not return */
        FUN_ram_0002fc40(uVar13,5,&DAT_ram_00034210);
      }
      ppuVar11 = local_68 + uVar13;
      uVar17 = 0xffffffffffffffff;
      if (*ppuVar11 < puVar6) {
        if (4 < uVar15 + uVar24) {
                    /* WARNING: Subroutine does not return */
          FUN_ram_0002fc40(0xffffffffffffffff,5,&DAT_ram_00034210);
        }
        uVar13 = uVar13 - 2;
        if (4 < uVar13) goto LAB_ram_0002c2f0;
        puVar22 = local_68[uVar15 + uVar24];
        FUN_ram_000334a8(&local_c8,puVar22,*ppuVar11,puVar6,0);
        uVar17 = local_c8;
        FUN_ram_00031e70(local_d8,local_c8,local_c0,puVar6,0);
        puVar27 = local_68[uVar13];
        puVar22 = puVar22 + -local_d8[0];
        do {
          FUN_ram_00031e70(&local_e8,uVar17,0,uVar5,0);
          bVar10 = true;
          bVar3 = true;
          if (puVar27 < local_e8) {
            bVar3 = false;
            if (local_e0 < puVar22) goto LAB_ram_0002b7a0;
LAB_ram_0002b7d8:
            bVar10 = false;
            if (puVar22 == local_e0) goto LAB_ram_0002b7e8;
LAB_ram_0002b7a8:
            if (bVar10) break;
          }
          else {
            if (puVar22 <= local_e0) goto LAB_ram_0002b7d8;
LAB_ram_0002b7a0:
            if (puVar22 != local_e0) goto LAB_ram_0002b7a8;
LAB_ram_0002b7e8:
            if (bVar3) break;
          }
          bVar3 = puVar22 <= puVar22 + (longlong)puVar6;
          uVar17 = uVar17 - 1;
          puVar22 = puVar22 + (longlong)puVar6;
        } while (bVar3);
      }
      FUN_ram_00031e70(&local_f8,uVar17,0,puVar9,0);
      FUN_ram_00031e70(&local_108,uVar17,0,puVar1,0);
      FUN_ram_00031e70(&local_118,uVar17,0,uVar16,0);
      FUN_ram_00031e70(&local_128,uVar17,0,uVar12,0);
      uVar13 = local_100 + (ulonglong)(local_f0 + local_108 < local_f0);
      local_a8 = uVar13 + local_118;
      uVar13 = local_110 + (ulonglong)(local_a8 < uVar13);
      local_a0 = uVar13 + local_128;
      local_b8[0] = local_f8;
      local_b8[1] = local_f0 + local_108;
      local_98 = local_120 + (ulonglong)(local_a0 < uVar13);
      if (5 < uVar19) {
        FUN_ram_00031038(uVar19,5,&DAT_ram_00034210);
        goto LAB_ram_0002c290;
      }
      uVar13 = 5 - uVar15;
      uVar21 = uVar13;
      if (uVar24 + 2 <= uVar13) {
        uVar21 = uVar24 + 2;
      }
      uVar2 = 5;
      if (uVar15 == 5) {
LAB_ram_0002c320:
                    /* WARNING: Subroutine does not return */
        FUN_ram_0002fc40(uVar2,4,&DAT_ram_00034210);
      }
      lVar23 = 0;
      uVar18 = 0;
      uVar2 = 0;
      do {
        uVar7 = *(ulonglong *)((longlong)local_b8 + lVar23);
        uVar2 = uVar7 + uVar2;
        uVar8 = *(ulonglong *)((longlong)local_190 + lVar23);
        *(ulonglong *)((longlong)local_190 + lVar23) = uVar8 - uVar2;
        uVar2 = (ulonglong)(uVar2 < uVar7 || uVar8 < uVar2);
        lVar23 = lVar23 + 8;
        uVar18 = uVar18 + 1;
      } while (uVar18 < uVar21);
      if (uVar2 != 0) {
        if (uVar14 <= uVar13) {
          uVar13 = uVar14;
        }
        lVar23 = 0;
        uVar21 = 0;
        uVar2 = 0;
        do {
          uVar18 = *(ulonglong *)((longlong)local_88 + lVar23);
          uVar2 = uVar18 + uVar2;
          uVar8 = *(ulonglong *)((longlong)local_190 + lVar23);
          uVar7 = uVar8 + uVar2;
          *(ulonglong *)((longlong)local_190 + lVar23) = uVar7;
          uVar2 = (ulonglong)(uVar2 < uVar18 || uVar7 < uVar8);
          lVar23 = lVar23 + 8;
          uVar21 = uVar21 + 1;
        } while (uVar21 < uVar13);
        *ppuVar11 = *ppuVar11 + uVar2;
        uVar17 = uVar17 - 1;
      }
      uVar2 = uVar19;
      if (3 < uVar19) goto LAB_ram_0002c320;
      local_40[uVar15] = uVar17;
      local_190 = local_190 + -1;
      bVar3 = uVar15 != 0;
      uVar15 = uVar15 - 1;
    } while (bVar3);
    local_98 = local_48;
    local_a0 = local_50;
    local_a8 = local_58;
    local_b8[1] = local_68[1];
    local_b8[0] = local_68[0];
    local_40[4] = (ulonglong)local_68[0] >> uVar4;
    local_40[5] = (ulonglong)local_68[1] >> uVar4;
    local_40[6] = local_58 >> uVar4;
    local_40[7] = local_50 >> uVar4;
    if (uVar26 != 0) {
      uVar15 = 1;
      do {
        if (uVar15 < 4) {
          uVar19 = uVar15 + 1;
          uVar16 = uVar15 - 1;
          if (3 < uVar16) {
                    /* WARNING: Subroutine does not return */
            FUN_ram_0002fc40(0xffffffffffffffff,4,&DAT_ram_00034210);
          }
        }
        else {
          uVar19 = 4;
          uVar16 = 3;
        }
        local_40[uVar16 + 4] = local_40[uVar16 + 4] | (longlong)local_b8[uVar15] << (-uVar26 & 0x3f)
        ;
        bVar3 = uVar15 < 4;
        uVar15 = uVar19;
      } while (bVar3);
    }
    param_1[7] = local_40[7];
    param_1[6] = local_40[6];
    param_1[5] = local_40[5];
    param_1[4] = local_40[4];
    param_1[3] = local_40[3];
    param_1[2] = local_40[2];
    param_1[1] = local_40[1];
    *param_1 = local_40[0];
  }
  return;
}

// Function: custom_panic
void custom_panic(longlong param_1)

{
  FUN_ram_0002c398(**(undefined8 **)(param_1 + 8),(*(undefined8 **)(param_1 + 8))[1]);
  FUN_ram_0002c3b8("** PANICKED **",0xe);
  return;
}

// Function: FUN_ram_0002c398
void FUN_ram_0002c398(void)

{
  FUN_ram_0002c398();
  FUN_ram_0002c3b8("** PANICKED **",0xe);
  return;
}

// Function: FUN_ram_0002c3b8
void FUN_ram_0002c3b8(void)

{
  FUN_ram_0002c3b8();
  return;
}

// Function: FUN_ram_0002c3c8
void FUN_ram_0002c3c8(ulonglong *param_1)

{
  *param_1 = *param_1 ^ 0x69d190c683eda5d3;
  param_1[1] = param_1[1] ^ 0x962f6f387c135a2c;
  param_1[2] = param_1[2] ^ 0x962c6f3b7c105a2d;
  param_1[3] = param_1[3] ^ 0x962d6f3a7c115a2e;
  param_1[4] = param_1[4] ^ 0x962a6f3d7c165a2f;
  param_1[5] = param_1[5] ^ 0x962b6f3c7c175a28;
  param_1[6] = param_1[6] ^ 0x96286f3f7c145a29;
  param_1[7] = param_1[7] ^ 0x96296f3e7c155a2a;
  param_1[8] = param_1[8] ^ 0x96266f317c1a5a2b;
  param_1[9] = param_1[9] ^ 0x96276f307c1b5a24;
  param_1[10] = param_1[10] ^ 0x96246f337c185a25;
  param_1[0xb] = param_1[0xb] ^ 0x96256f327c195a26;
  param_1[0xc] = param_1[0xc] ^ 0x96226f357c1e5a27;
  param_1[0xd] = param_1[0xd] ^ 0x96236f347c1f5a20;
  param_1[0xe] = param_1[0xe] ^ 0x96206f377c1c5a21;
  param_1[0xf] = param_1[0xf] ^ 0x96216f367c1d5a22;
  param_1[0x10] = param_1[0x10] ^ 0x963e6f297c025a23;
  param_1[0x11] = param_1[0x11] ^ 0x963f6f287c035a3c;
  FUN_ram_00002a50(param_1 + 0x12);
  param_1[0x30] = param_1[0x30] ^ 0xfb5ce87aae443c38;
  param_1[0x31] = param_1[0x31] ^ 0x4a2178451bac3c7;
  param_1[0x32] = param_1[0x32] ^ 0x4a1178751b9c3c6;
  param_1[0x33] = param_1[0x33] ^ 0x4a0178651b8c3c5;
  param_1[0x34] = param_1[0x34] ^ 0xfb5ce87aae443c38;
  param_1[0x35] = param_1[0x35] ^ 0x4a2178451bac3c7;
  param_1[0x36] = param_1[0x36] ^ 0x4a1178751b9c3c6;
  param_1[0x37] = param_1[0x37] ^ 0x4a0178651b8c3c5;
  param_1[0x38] = param_1[0x38] ^ 0xfb5ce87aae443c38;
  param_1[0x39] = param_1[0x39] ^ 0x4a2178451bac3c7;
  param_1[0x3a] = param_1[0x3a] ^ 0x4a1178751b9c3c6;
  param_1[0x3b] = param_1[0x3b] ^ 0x4a0178651b8c3c5;
  param_1[0x3c] = param_1[0x3c] ^ 0xfb5ce87aae443c38;
  param_1[0x3d] = param_1[0x3d] ^ 0x4a2178451bac3c7;
  param_1[0x3e] = param_1[0x3e] ^ 0x4a1178751b9c3c6;
  param_1[0x3f] = param_1[0x3f] ^ 0x4a0178651b8c3c5;
  param_1[0x40] = param_1[0x40] ^ 0xfb5ce87aae443c38;
  param_1[0x41] = param_1[0x41] ^ 0x4a2178451bac3c7;
  param_1[0x42] = param_1[0x42] ^ 0x4a1178751b9c3c6;
  param_1[0x43] = param_1[0x43] ^ 0x4a0178651b8c3c5;
  param_1[0x44] = param_1[0x44] ^ 0xb957ed15dc877c26;
  param_1[0x45] = param_1[0x45] ^ 0x46a912eb237873d9;
  param_1[0x46] = param_1[0x46] ^ 0xf539f2cf9513d4a1;
  param_1[0x47] = param_1[0x47] ^ 0xadcf8e5743314562;
  param_1[0x48] = param_1[0x48] ^ 0xb957ed15dc877426;
  param_1[0x49] = param_1[0x49] ^ 0x46a912eb23798bd9;
  param_1[0x4a] = param_1[0x4a] ^ 0x6e9de2b30b19f9ea;
  param_1[0x4b] = param_1[0x4b] ^ 0x6e9de2b30b19f9ea;
  param_1[0x4c] = param_1[0x4c] ^ 0x6e9de2b30b19f1ea;
  param_1[0x4d] = param_1[0x4d] ^ 0x6e9de2b30b19f1ea;
  param_1[0x4e] = param_1[0x4e] ^ 0xdbf169454ad22fa;
  param_1[0x4f] = param_1[0x4f] ^ 0xf241e96aab522d05;
  param_1[0x50] = param_1[0x50] ^ 0xf242e969ab532d04;
  param_1[0x51] = param_1[0x51] ^ 0xf243e968ab502d07;
  param_1[0x52] = param_1[0x52] ^ 0xf244e96fab512d06;
  param_1[0x53] = param_1[0x53] ^ 0xf245e96eab562d01;
  param_1[0x54] = param_1[0x54] ^ 0xf246e96dab572d00;
  param_1[0x55] = param_1[0x55] ^ 0xf247e96cab542d03;
  param_1[0x56] = param_1[0x56] ^ 0xf248e963ab552d02;
  param_1[0x57] = param_1[0x57] ^ 0xf249e962ab5a2d0d;
  param_1[0x58] = param_1[0x58] ^ 0xed5f563e78eee80b;
  param_1[0x59] = param_1[0x59] ^ 0xd3198133b7c1776c;
  param_1[0x5a] = param_1[0x5a] ^ 0xb82c93d08854ebff;
  param_1[0x5b] = param_1[0x5b] ^ 0x47d26c2e77aa1400;
  param_1[0x5c] = param_1[0x5c] ^ 0x47d16c2d77a91401;
  param_1[0x5d] = param_1[0x5d] ^ 0x47d06c2c77a81402;
  param_1[0x5e] = param_1[0x5e] ^ 0x47d76c2b77af1403;
  param_1[0x5f] = param_1[0x5f] ^ 0x47d66c2a77ae1404;
  param_1[0x60] = param_1[0x60] ^ 0x47d56c2977ad1405;
  param_1[0x61] = param_1[0x61] ^ 0x47d46c2877ac1406;
  param_1[0x62] = param_1[0x62] ^ 0x47db6c2777a31407;
  param_1[99] = param_1[99] ^ 0x47da6c2677a21408;
  param_1[100] = param_1[100] ^ 0x504156a22548f8dd;
  param_1[0x65] = param_1[0x65] ^ 0x35f72d643d3464eb;
  param_1[0x66] = param_1[0x66] ^ 0x9578e14d1d0d9c4e;
  param_1[0x67] = param_1[0x67] ^ 0xff64577ac49fae40;
  param_1[0x68] = param_1[0x68] ^ 0x9aa8843b60a9bf;
  param_1[0x69] = param_1[0x69] ^ 0x99a8873b61a9be;
  param_1[0x6a] = param_1[0x6a] ^ 0x98a8863b62a9bd;
  param_1[0x6b] = param_1[0x6b] ^ 0x9fa8813b63a9bc;
  param_1[0x6c] = param_1[0x6c] ^ 0x9ea8803b64a9bb;
  param_1[0x6d] = param_1[0x6d] ^ 0x9da8833b65a9ba;
  param_1[0x6e] = param_1[0x6e] ^ 0x9ca8823b66a9b9;
  param_1[0x6f] = param_1[0x6f] ^ 0x93a88d3b67a9b8;
  param_1[0x70] = param_1[0x70] ^ 0x92a88c3b68a9b7;
  param_1[0x71] = param_1[0x71] ^ 0xfb5ce87aae443c38;
  param_1[0x72] = param_1[0x72] ^ 0x4a2178451bac3c7;
  param_1[0x73] = param_1[0x73] ^ 0x4a1178751b9c3c6;
  param_1[0x74] = param_1[0x74] ^ 0x4a0178651b8c3c5;
  param_1[0x75] = param_1[0x75] ^ 0xcf44133cb352d91c;
  return;
}

// Function: FUN_ram_0002d050
void FUN_ram_0002d050(undefined8 *param_1,longlong param_2,ulonglong *param_3,longlong param_4)

{
  bool bVar1;
  bool bVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong *puVar5;
  undefined8 uVar6;
  ulonglong local_18;
  longlong local_10;
  
  if (*(char *)(param_2 + 8) != '\0') {
    uVar6 = 0;
    FUN_ram_0002df08(&local_18,(ulonglong)*(ushort *)(param_2 + 10) << 0x30,0,0x3e8000000000000);
    goto LAB_ram_0002d278;
  }
  uVar6 = 1;
  if ((param_4 == 0) || (*param_3 == 0)) {
LAB_ram_0002d268:
    local_18 = 0;
    local_10 = 0;
    goto LAB_ram_0002d278;
  }
  if (*(ulonglong *)(param_2 + 0x30) == 0) {
    uVar4 = 0;
  }
  else {
    uVar3 = *(ulonglong *)(param_2 + 0x40) / 0x34bc0;
    if (0x16c < uVar3) {
      uVar3 = 0x16d;
    }
    uVar4 = 1;
    if (0x34bbf < *(ulonglong *)(param_2 + 0x40)) {
      uVar4 = uVar3;
    }
    uVar4 = uVar4 * (*(ulonglong *)(param_2 + 0x30) / 1000000);
  }
  param_4 = param_4 << 4;
  do {
    puVar5 = param_3;
    if (param_4 == 0) goto LAB_ram_0002d268;
    param_4 = param_4 + -0x10;
    param_3 = puVar5 + 2;
  } while (uVar4 < *puVar5);
  FUN_ram_0002df08(&local_18,puVar5[1] << 0x30,puVar5[1] >> 0x10,0x4240000000000000);
  uVar6 = 1;
  bVar1 = true;
  if (local_18 < 0x800000000000) {
    if (-1 < local_10) goto LAB_ram_0002d2b8;
LAB_ram_0002d248:
    bVar2 = true;
    if (local_10 != 0) goto LAB_ram_0002d2c8;
LAB_ram_0002d250:
    if (!bVar1) goto LAB_ram_0002d2d8;
LAB_ram_0002d258:
    if (bVar1) goto LAB_ram_0002d278;
  }
  else {
    bVar1 = false;
    if (local_10 < 0) goto LAB_ram_0002d248;
LAB_ram_0002d2b8:
    bVar2 = false;
    if (local_10 == 0) goto LAB_ram_0002d250;
LAB_ram_0002d2c8:
    bVar1 = bVar2;
    if (bVar1) goto LAB_ram_0002d258;
LAB_ram_0002d2d8:
    local_10 = 0;
    if (bVar1) goto LAB_ram_0002d278;
  }
  local_18 = 0x800000000000;
LAB_ram_0002d278:
  param_1[1] = local_18;
  *param_1 = uVar6;
  param_1[2] = local_10;
  return;
}

// Function: FUN_ram_0002d300
/* WARNING: Removing unreachable block (ram,0x0002d620) */

void FUN_ram_0002d300(undefined4 *param_1,undefined8 *param_2,undefined8 param_3,undefined8 param_4)

{
  char cVar1;
  byte bVar2;
  bool bVar3;
  undefined1 *puVar4;
  longlong lVar5;
  undefined4 uVar6;
  longlong lVar7;
  undefined1 uVar8;
  longlong *local_e0;
  undefined2 local_d8;
  longlong *local_d0;
  undefined2 local_c8;
  undefined1 local_b9;
  undefined1 local_b8;
  undefined1 local_b7;
  undefined1 local_b6;
  undefined1 local_b5;
  undefined1 local_b4;
  undefined1 local_b3;
  undefined1 local_b2;
  undefined1 local_b1;
  undefined1 local_b0;
  undefined1 local_af;
  undefined1 local_ae;
  undefined1 local_ad;
  undefined1 local_ac;
  undefined1 local_ab;
  undefined1 local_aa;
  undefined1 local_a9;
  undefined1 local_a8;
  undefined1 local_a7;
  undefined1 local_a6;
  undefined1 local_a5;
  undefined1 local_a4;
  undefined1 local_a3;
  undefined1 local_a2;
  undefined1 local_a1;
  undefined1 local_a0;
  undefined1 local_9f;
  undefined1 local_9e;
  undefined1 local_9d;
  undefined1 local_9c;
  undefined1 local_9b;
  undefined1 local_9a;
  undefined1 local_99;
  char *local_98;
  char *local_90;
  undefined8 local_88;
  char *local_80;
  char *local_78;
  undefined8 local_70;
  undefined1 local_68;
  undefined1 local_67;
  undefined1 local_66;
  byte *local_60;
  byte *local_58;
  undefined8 local_50;
  byte *local_48;
  byte *local_40;
  undefined8 local_38;
  undefined1 local_30;
  undefined1 local_2f;
  undefined1 local_2e;
  undefined *local_28;
  longlong **local_20;
  undefined8 local_18;
  undefined1 *local_10;
  undefined8 local_8;
  
  lVar5 = *(longlong *)param_2[1];
  local_d0 = (longlong *)(lVar5 + 8);
  lVar7 = *(longlong *)*param_2;
  local_e0 = (longlong *)(lVar7 + 8);
  local_c8 = 0;
  local_d8 = 1;
  local_b9 = 0x12;
  puVar4 = (undefined1 *)param_2[2];
  local_b8 = *puVar4;
  local_b7 = puVar4[1];
  local_b6 = puVar4[2];
  local_b5 = puVar4[3];
  local_b4 = puVar4[4];
  local_b3 = puVar4[5];
  local_b2 = puVar4[6];
  local_b1 = puVar4[7];
  local_b0 = puVar4[8];
  local_af = puVar4[9];
  local_ae = puVar4[10];
  local_ad = puVar4[0xb];
  local_ac = puVar4[0xc];
  local_ab = puVar4[0xd];
  local_aa = puVar4[0xe];
  local_a9 = puVar4[0xf];
  local_a8 = puVar4[0x10];
  local_a7 = puVar4[0x11];
  local_a6 = puVar4[0x12];
  local_a5 = puVar4[0x13];
  local_a4 = puVar4[0x14];
  local_a3 = puVar4[0x15];
  local_a2 = puVar4[0x16];
  local_a1 = puVar4[0x17];
  local_a0 = puVar4[0x18];
  local_9f = puVar4[0x19];
  local_9e = puVar4[0x1a];
  local_9d = puVar4[0x1b];
  local_9c = puVar4[0x1c];
  local_9b = puVar4[0x1d];
  local_9a = puVar4[0x1e];
  local_99 = puVar4[0x1f];
  local_90 = *(char **)*param_2;
  local_98 = local_90 + 8;
  if ((((*(longlong *)(local_90 + 8) != *local_e0) ||
       (*(longlong *)(local_90 + 0x10) != *(longlong *)(lVar7 + 0x10))) ||
      (*(longlong *)(local_90 + 0x18) != *(longlong *)(lVar7 + 0x18))) ||
     (bVar3 = false, *(longlong *)(local_90 + 0x20) != *(longlong *)(lVar7 + 0x20))) {
    bVar3 = true;
  }
  uVar6 = 1;
  if ((bVar3) || (uVar6 = 0xb, *local_90 != -1)) goto LAB_ram_0002d980;
  local_68 = 1;
  if (local_90[1] == '\0') {
    local_68 = 0;
    if (local_90[2] != '\0') goto LAB_ram_0002d690;
LAB_ram_0002d9b0:
    uVar8 = 0;
    local_67 = 0;
    cVar1 = local_90[3];
  }
  else {
    if (local_90[2] == '\0') goto LAB_ram_0002d9b0;
LAB_ram_0002d690:
    uVar8 = 1;
    local_67 = 1;
    cVar1 = local_90[3];
  }
  if (cVar1 == '\0') {
    local_67 = uVar8;
  }
  local_66 = cVar1 != '\0';
  local_88 = *(undefined8 *)(local_90 + 0x50);
  local_78 = local_90 + 0x28;
  local_80 = local_90 + 0x58;
  local_90 = local_90 + 0x48;
  local_70 = 0;
  local_58 = *(byte **)param_2[1];
  local_60 = local_58 + 8;
  if (((*(longlong *)(local_58 + 8) != *local_d0) ||
      (*(longlong *)(local_58 + 0x10) != *(longlong *)(lVar5 + 0x10))) ||
     ((*(longlong *)(local_58 + 0x18) != *(longlong *)(lVar5 + 0x18) ||
      (bVar3 = false, *(longlong *)(local_58 + 0x20) != *(longlong *)(lVar5 + 0x20))))) {
    bVar3 = true;
  }
  uVar6 = 1;
  if ((bVar3) || (uVar6 = 0xb, (*local_58 | 0x77) != 0xff)) goto LAB_ram_0002d980;
  local_30 = 1;
  if (local_58[1] == 0) {
    local_30 = 0;
    if (local_58[2] != 0) goto LAB_ram_0002d850;
LAB_ram_0002d9f0:
    uVar8 = 0;
    local_2f = 0;
    bVar2 = local_58[3];
  }
  else {
    if (local_58[2] == 0) goto LAB_ram_0002d9f0;
LAB_ram_0002d850:
    uVar8 = 1;
    local_2f = 1;
    bVar2 = local_58[3];
  }
  if (bVar2 == 0) {
    local_2f = uVar8;
  }
  local_2e = bVar2 != 0;
  local_50 = *(undefined8 *)(local_58 + 0x50);
  local_40 = local_58 + 0x28;
  local_48 = local_58 + 0x58;
  local_58 = local_58 + 0x48;
  local_38 = 0;
  local_10 = &local_b9;
  local_20 = &local_e0;
  local_28 = &DAT_ram_00033580;
  local_8 = 0x21;
  local_18 = 2;
  FUN_ram_0002d968(&local_28,&local_98,2,param_3,param_4);
  uVar6 = 0x1a;
LAB_ram_0002d980:
  *param_1 = uVar6;
  return;
}

// Function: FUN_ram_0002d968
void FUN_ram_0002d968(void)

{
  undefined4 *unaff_R6;
  
  FUN_ram_0002d968();
  *unaff_R6 = 0x1a;
  return;
}

// Function: FUN_ram_0002da18
void FUN_ram_0002da18(undefined4 *param_1,undefined8 *param_2,undefined8 param_3,undefined8 param_4)

{
  char cVar1;
  byte bVar2;
  undefined4 uVar3;
  undefined1 uVar4;
  char *local_110;
  undefined2 local_108;
  char *local_100;
  undefined2 local_f8;
  byte *local_f0;
  undefined2 local_e8;
  undefined1 local_d9;
  undefined8 local_d8;
  char *local_d0;
  char *local_c8;
  undefined8 local_c0;
  char *local_b8;
  char *local_b0;
  undefined8 local_a8;
  undefined1 local_a0;
  undefined1 local_9f;
  undefined1 local_9e;
  char *local_98;
  char *local_90;
  undefined8 local_88;
  char *local_80;
  char *local_78;
  undefined8 local_70;
  undefined1 local_68;
  undefined1 local_67;
  undefined1 local_66;
  byte *local_60;
  byte *local_58;
  undefined8 local_50;
  byte *local_48;
  byte *local_40;
  undefined8 local_38;
  undefined1 local_30;
  undefined1 local_2f;
  undefined1 local_2e;
  undefined *local_28;
  char **local_20;
  undefined8 local_18;
  undefined1 *local_10;
  undefined8 local_8;
  
  local_c8 = *(char **)*param_2;
  local_90 = *(char **)param_2[1];
  local_58 = *(byte **)param_2[2];
  local_f0 = local_58 + 8;
  local_100 = local_90 + 8;
  local_110 = local_c8 + 8;
  local_e8 = 0x100;
  local_f8 = 1;
  local_108 = 1;
  local_d9 = 3;
  local_d8 = param_2[3];
  if (*local_c8 == -1) {
    local_a0 = local_c8[1] != '\0';
    local_9f = local_c8[2] != '\0';
    local_9e = local_c8[3] != '\0';
    local_c0 = *(undefined8 *)(local_c8 + 0x50);
    local_b0 = local_c8 + 0x28;
    local_b8 = local_c8 + 0x58;
    local_c8 = local_c8 + 0x48;
    local_a8 = 0;
    if (*local_90 == -1) {
      local_68 = 1;
      if (local_90[1] == '\0') {
        local_68 = 0;
        if (local_90[2] == '\0') goto LAB_ram_0002dea0;
LAB_ram_0002dc28:
        uVar4 = 1;
        local_67 = 1;
        cVar1 = local_90[3];
      }
      else {
        if (local_90[2] != '\0') goto LAB_ram_0002dc28;
LAB_ram_0002dea0:
        uVar4 = 0;
        local_67 = 0;
        cVar1 = local_90[3];
      }
      if (cVar1 == '\0') {
        local_67 = uVar4;
      }
      local_66 = cVar1 != '\0';
      local_88 = *(undefined8 *)(local_90 + 0x50);
      local_78 = local_90 + 0x28;
      local_80 = local_90 + 0x58;
      local_90 = local_90 + 0x48;
      local_70 = 0;
      if ((*local_58 & 0x88) == 0x88) {
        local_30 = 1;
        if (local_58[1] == 0) {
          local_30 = 0;
          if (local_58[2] == 0) goto LAB_ram_0002dee0;
LAB_ram_0002dd20:
          uVar4 = 1;
          local_2f = 1;
          bVar2 = local_58[3];
        }
        else {
          if (local_58[2] != 0) goto LAB_ram_0002dd20;
LAB_ram_0002dee0:
          uVar4 = 0;
          local_2f = 0;
          bVar2 = local_58[3];
        }
        if (bVar2 == 0) {
          local_2f = uVar4;
        }
        local_2e = bVar2 != 0;
        local_50 = *(undefined8 *)(local_58 + 0x50);
        local_40 = local_58 + 0x28;
        local_48 = local_58 + 0x58;
        local_58 = local_58 + 0x48;
        local_38 = 0;
        local_10 = &local_d9;
        local_20 = &local_110;
        local_28 = &DAT_ram_00033580;
        local_8 = 9;
        local_18 = 3;
        local_d0 = local_110;
        local_98 = local_100;
        local_60 = local_f0;
        FUN_ram_0002de40(&local_28,&local_d0,3,param_3,param_4);
        uVar3 = 0x1a;
        goto LAB_ram_0002de68;
      }
    }
  }
  uVar3 = 0xb;
LAB_ram_0002de68:
  *param_1 = uVar3;
  return;
}

// Function: FUN_ram_0002de40
void FUN_ram_0002de40(void)

{
  undefined4 *unaff_R6;
  
  FUN_ram_0002de40();
  *unaff_R6 = 0x1a;
  return;
}

// Function: FUN_ram_0002df08
/* WARNING: Type propagation algorithm not settling */

void FUN_ram_0002df08(longlong *param_1,longlong param_2,ulonglong param_3,ulonglong param_4,
                     longlong param_5)

{
  longlong lVar1;
  ulonglong uVar2;
  longlong lVar3;
  ulonglong uVar4;
  longlong local_50;
  ulonglong local_48;
  longlong local_40;
  longlong local_38;
  longlong local_30;
  longlong local_28;
  ulonglong local_20;
  ulonglong local_18;
  
  uVar2 = *(ulonglong *)(param_5 + -0x1000);
  uVar4 = *(ulonglong *)(param_5 + -0xff8);
  if ((uVar4 & 0xffffffff) == 0x80) {
    local_40 = 0;
    if (param_2 == 0 && param_3 == 0x8000000000000000) {
      local_38 = 0;
      if ((param_4 & uVar2) == 0xffffffffffffffff) {
        param_1[1] = 0;
        *param_1 = 0;
        *(undefined1 *)(param_1 + 2) = 1;
        return;
      }
    }
    else {
      local_38 = 0;
    }
  }
  else {
    FUN_ram_00031e28(&local_40,param_2,param_3,uVar4 & 0x7f);
    FUN_ram_00031b90(&local_50,param_2,param_3,-uVar4 & 0x7f);
    param_3 = local_48;
    param_2 = local_50;
  }
  uVar4 = 0xffffffffffffffff;
  lVar3 = -1;
  if ((local_40 == 0 && local_38 == 0) && (lVar3 = 0, param_2 != 0)) {
    lVar3 = 0;
  }
  lVar1 = local_40;
  if ((longlong)param_3 < 0) {
    lVar1 = -local_40;
  }
  if ((longlong)param_3 < 0) {
    local_38 = -(local_38 + (ulonglong)(local_40 != 0));
  }
  lVar3 = lVar3 - param_2;
  if (-1 < (longlong)param_3) {
    lVar3 = param_2;
  }
  FUN_ram_0002e490(&local_30,lVar1,local_38,lVar3);
  if (local_30 == 0 && local_28 == 0) {
    uVar4 = 0;
  }
  param_3 = param_3 ^ uVar2;
  if ((longlong)param_3 < 0) {
    local_18 = (uVar4 - local_18) - (ulonglong)(uVar4 < local_20);
    local_20 = uVar4 - local_20;
  }
  if ((longlong)param_3 < 0) {
    local_28 = -(local_28 + (ulonglong)(local_30 != 0));
  }
  lVar3 = -local_30;
  if (-1 < (longlong)param_3) {
    lVar3 = local_30;
  }
  *param_1 = lVar3;
  param_1[1] = local_28;
  *(bool *)(param_1 + 2) = local_20 != local_28 >> 0x3f || local_18 != local_28 >> 0x3f;
  return;
}

// Function: FUN_ram_0002e490
void FUN_ram_0002e490(ulonglong *param_1,ulonglong param_2,ulonglong param_3,ulonglong param_4,
                     longlong param_5)

{
  ulonglong uVar1;
  longlong lVar2;
  bool bVar3;
  longlong lVar4;
  bool bVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  longlong lVar11;
  undefined4 uVar12;
  longlong lVar13;
  ulonglong uVar14;
  longlong local_178;
  ulonglong local_148;
  ulonglong local_140;
  ulonglong local_138;
  ulonglong local_128;
  ulonglong local_120;
  ulonglong local_118;
  ulonglong local_110;
  ulonglong local_108;
  longlong local_100 [2];
  ulonglong local_f0;
  longlong local_e8;
  ulonglong local_e0;
  ulonglong local_d8;
  longlong local_d0 [2];
  longlong local_c0;
  undefined8 local_b8;
  ulonglong local_b0;
  ulonglong local_a8;
  longlong local_a0 [2];
  ulonglong local_90;
  longlong local_88;
  ulonglong local_80;
  ulonglong local_78;
  longlong local_70 [2];
  longlong local_60;
  undefined8 local_58;
  ulonglong local_50;
  ulonglong local_48;
  ulonglong local_40;
  ulonglong local_38;
  ulonglong local_30;
  ulonglong local_28;
  ulonglong local_20;
  ulonglong local_18;
  longlong local_10;
  undefined8 local_8;
  
  local_128 = *(ulonglong *)(param_5 + -0xff8);
  uVar9 = *(ulonglong *)(param_5 + -0xff0);
  if (uVar9 == 0) {
    uVar1 = local_128 | local_128 >> 1;
    uVar1 = uVar1 | uVar1 >> 2;
    uVar1 = uVar1 | uVar1 >> 4;
    uVar1 = uVar1 | uVar1 >> 8;
    uVar1 = uVar1 | uVar1 >> 0x10;
    uVar1 = (uVar1 | uVar1 >> 0x20) ^ 0xffffffffffffffff;
    uVar1 = uVar1 - (uVar1 >> 1 & 0x5555555555555555);
    uVar1 = (uVar1 & 0x3333333333333333) + (uVar1 >> 2 & 0x3333333333333333);
    uVar1 = ((uVar1 + (uVar1 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38) + 0x40;
  }
  else {
    uVar1 = uVar9 | uVar9 >> 1;
    uVar1 = uVar1 | uVar1 >> 2;
    uVar1 = uVar1 | uVar1 >> 4;
    uVar1 = uVar1 | uVar1 >> 8;
    uVar1 = uVar1 | uVar1 >> 0x10;
    uVar1 = (uVar1 | uVar1 >> 0x20) ^ 0xffffffffffffffff;
    uVar1 = uVar1 - (uVar1 >> 1 & 0x5555555555555555);
    uVar1 = (uVar1 & 0x3333333333333333) + (uVar1 >> 2 & 0x3333333333333333);
    uVar1 = (uVar1 + (uVar1 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38;
  }
  uVar10 = *(ulonglong *)(param_5 + -0x1000);
  uVar12 = (undefined4)uVar1;
  if (uVar1 == 0) {
    local_10 = 0;
    local_8 = 0;
    local_148 = param_4;
    local_140 = param_3;
    local_138 = param_2;
  }
  else {
    FUN_ram_00032018(&local_10,param_4,uVar10,-uVar1 & 0x7f);
    FUN_ram_00031e28(&local_20,param_4,uVar10,uVar12);
    FUN_ram_00032018(&local_30,param_2,param_3,-uVar1 & 0x7f);
    FUN_ram_00031e28(&local_40,param_2,param_3,uVar12);
    FUN_ram_00031e28(&local_50,local_128,uVar9,uVar12);
    uVar10 = local_28 | local_18;
    local_148 = local_30 | local_20;
    local_128 = local_50;
    local_140 = local_38;
    local_138 = local_40;
    uVar9 = local_48;
  }
  FUN_ram_000334a8(&local_60,local_10,local_8,uVar9,0);
  FUN_ram_00031e70(local_70,local_60,local_58,uVar9,0);
  FUN_ram_00031e70(&local_80,local_60,local_58,local_128,0);
  uVar1 = local_10 - local_70[0];
  bVar5 = true;
  bVar3 = true;
  if (uVar10 < local_80) {
    bVar3 = false;
    if (uVar1 < local_78) goto LAB_ram_0002ebb8;
LAB_ram_0002ea38:
    if (uVar1 == local_78) goto LAB_ram_0002ea40;
LAB_ram_0002ebc8:
    if (bVar5) goto LAB_ram_0002ebe0;
LAB_ram_0002ea50:
    uVar8 = uVar10 + local_128;
    uVar7 = uVar1 + uVar9 + (ulonglong)(uVar8 < uVar10);
    uVar6 = 1;
    if (uVar8 < local_80) {
      uVar6 = 0;
      if (uVar7 < local_78) goto LAB_ram_0002f668;
LAB_ram_0002eac8:
      uVar14 = 1;
      if (uVar7 == local_78) goto LAB_ram_0002ead0;
LAB_ram_0002f678:
      uVar6 = uVar14;
      if (uVar1 <= uVar7) goto LAB_ram_0002f690;
LAB_ram_0002eae0:
      uVar14 = 1;
      if (uVar7 == uVar1) goto LAB_ram_0002eae8;
LAB_ram_0002f6a0:
      uVar6 = uVar14 | uVar6;
    }
    else {
      if (local_78 <= uVar7) goto LAB_ram_0002eac8;
LAB_ram_0002f668:
      uVar14 = 0;
      if (uVar7 != local_78) goto LAB_ram_0002f678;
LAB_ram_0002ead0:
      if (uVar7 < uVar1) goto LAB_ram_0002eae0;
LAB_ram_0002f690:
      uVar14 = 0;
      if (uVar7 != uVar1) goto LAB_ram_0002f6a0;
LAB_ram_0002eae8:
      uVar6 = uVar8 < uVar10 | uVar6;
    }
    lVar11 = -1;
    if (uVar6 == 0) {
      lVar11 = -2;
    }
    uVar1 = uVar9;
    if ((uVar6 == 0) && (uVar8 = local_128 + uVar8, uVar8 < local_128)) {
      lVar4 = 1;
    }
    else {
      lVar4 = 0;
      if (uVar6 != 0) {
        uVar1 = 0;
      }
    }
    lVar4 = (uVar1 + uVar7 + lVar4) - local_78;
    lVar11 = lVar11 + local_60;
    uVar10 = uVar8;
  }
  else {
    if (local_78 <= uVar1) goto LAB_ram_0002ea38;
LAB_ram_0002ebb8:
    bVar5 = false;
    if (uVar1 != local_78) goto LAB_ram_0002ebc8;
LAB_ram_0002ea40:
    if (!bVar3) goto LAB_ram_0002ea50;
LAB_ram_0002ebe0:
    lVar4 = uVar1 - local_78;
    lVar11 = local_60;
  }
  bVar3 = true;
  FUN_ram_000334a8(&local_90,uVar10 - local_80,lVar4 - (ulonglong)(uVar10 < local_80),uVar9,0);
  FUN_ram_00031e70(local_a0,local_90,local_88,uVar9,0);
  FUN_ram_00031e70(&local_b0,local_90,local_88,local_128,0);
  uVar1 = (uVar10 - local_80) - local_a0[0];
  bVar5 = true;
  if (local_148 < local_b0) {
    bVar5 = false;
    if (uVar1 < local_a8) goto LAB_ram_0002ef10;
LAB_ram_0002ed28:
    if (uVar1 == local_a8) goto LAB_ram_0002ed30;
LAB_ram_0002ef28:
    if (bVar3) goto LAB_ram_0002ef40;
LAB_ram_0002ed40:
    uVar7 = local_148 + local_128;
    uVar6 = uVar1 + uVar9 + (ulonglong)(uVar7 < local_148);
    uVar10 = 1;
    if (uVar7 < local_b0) {
      uVar10 = 0;
      if (uVar6 < local_a8) goto LAB_ram_0002f720;
LAB_ram_0002edc0:
      uVar8 = 1;
      if (uVar6 == local_a8) goto LAB_ram_0002edc8;
LAB_ram_0002f730:
      uVar10 = uVar8;
      if (uVar1 <= uVar6) goto LAB_ram_0002f748;
LAB_ram_0002edd8:
      uVar8 = 1;
      if (uVar6 == uVar1) goto LAB_ram_0002ede0;
LAB_ram_0002f758:
      uVar10 = uVar8 | uVar10;
    }
    else {
      if (local_a8 <= uVar6) goto LAB_ram_0002edc0;
LAB_ram_0002f720:
      uVar8 = 0;
      if (uVar6 != local_a8) goto LAB_ram_0002f730;
LAB_ram_0002edc8:
      if (uVar6 < uVar1) goto LAB_ram_0002edd8;
LAB_ram_0002f748:
      uVar8 = 0;
      if (uVar6 != uVar1) goto LAB_ram_0002f758;
LAB_ram_0002ede0:
      uVar10 = uVar7 < local_148 | uVar10;
    }
    uVar8 = 0xffffffffffffffff;
    if (uVar10 == 0) {
      uVar8 = 0xfffffffffffffffe;
    }
    uVar1 = uVar8 + local_90;
    uVar14 = 0;
    if (uVar10 == 0) {
      uVar14 = local_128;
    }
    local_148 = uVar14 + uVar7;
    uVar7 = 0;
    if (uVar10 == 0) {
      uVar7 = uVar9;
    }
    lVar2 = local_88 + (ulonglong)(uVar1 < uVar8) + -1;
    lVar4 = (uVar7 + uVar6 + (ulonglong)(local_148 < uVar14)) - local_a8;
  }
  else {
    if (local_a8 <= uVar1) goto LAB_ram_0002ed28;
LAB_ram_0002ef10:
    bVar3 = false;
    if (uVar1 != local_a8) goto LAB_ram_0002ef28;
LAB_ram_0002ed30:
    if (!bVar5) goto LAB_ram_0002ed40;
LAB_ram_0002ef40:
    lVar4 = uVar1 - local_a8;
    lVar2 = local_88;
    uVar1 = local_90;
  }
  bVar3 = true;
  FUN_ram_000334a8(&local_c0,local_148 - local_b0,lVar4 - (ulonglong)(local_148 < local_b0),uVar9,0)
  ;
  FUN_ram_00031e70(local_d0,local_c0,local_b8,uVar9,0);
  FUN_ram_00031e70(&local_e0,local_c0,local_b8,local_128,0);
  uVar10 = (local_148 - local_b0) - local_d0[0];
  bVar5 = true;
  if (local_140 < local_e0) {
    bVar5 = false;
    if (uVar10 < local_d8) goto LAB_ram_0002f218;
LAB_ram_0002f078:
    if (uVar10 == local_d8) goto LAB_ram_0002f080;
LAB_ram_0002f230:
    if (bVar3) goto LAB_ram_0002f248;
LAB_ram_0002f090:
    uVar8 = local_140 + local_128;
    uVar7 = uVar10 + uVar9 + (ulonglong)(uVar8 < local_140);
    uVar6 = 1;
    if (uVar8 < local_e0) {
      uVar6 = 0;
      if (uVar7 < local_d8) goto LAB_ram_0002f798;
LAB_ram_0002f108:
      uVar14 = 1;
      if (uVar7 == local_d8) goto LAB_ram_0002f110;
LAB_ram_0002f7a8:
      uVar6 = uVar14;
      if (uVar10 <= uVar7) goto LAB_ram_0002f7c0;
LAB_ram_0002f120:
      uVar14 = 1;
      if (uVar7 == uVar10) goto LAB_ram_0002f128;
LAB_ram_0002f7d0:
      uVar6 = uVar14 | uVar6;
    }
    else {
      if (local_d8 <= uVar7) goto LAB_ram_0002f108;
LAB_ram_0002f798:
      uVar14 = 0;
      if (uVar7 != local_d8) goto LAB_ram_0002f7a8;
LAB_ram_0002f110:
      if (uVar7 < uVar10) goto LAB_ram_0002f120;
LAB_ram_0002f7c0:
      uVar14 = 0;
      if (uVar7 != uVar10) goto LAB_ram_0002f7d0;
LAB_ram_0002f128:
      uVar6 = uVar8 < local_140 | uVar6;
    }
    lVar13 = -1;
    if (uVar6 == 0) {
      lVar13 = -2;
    }
    lVar13 = lVar13 + local_c0;
    uVar10 = 0;
    if (uVar6 == 0) {
      uVar10 = local_128;
    }
    local_140 = uVar10 + uVar8;
    uVar8 = 0;
    if (uVar6 == 0) {
      uVar8 = uVar9;
    }
    lVar4 = (uVar8 + uVar7 + (ulonglong)(local_140 < uVar10)) - local_d8;
  }
  else {
    if (local_d8 <= uVar10) goto LAB_ram_0002f078;
LAB_ram_0002f218:
    bVar3 = false;
    if (uVar10 != local_d8) goto LAB_ram_0002f230;
LAB_ram_0002f080:
    if (!bVar5) goto LAB_ram_0002f090;
LAB_ram_0002f248:
    lVar4 = uVar10 - local_d8;
    lVar13 = local_c0;
  }
  bVar3 = true;
  FUN_ram_000334a8(&local_f0,local_140 - local_e0,lVar4 - (ulonglong)(local_140 < local_e0),uVar9,0)
  ;
  FUN_ram_00031e70(local_100,local_f0,local_e8,uVar9,0);
  FUN_ram_00031e70(&local_110,local_f0,local_e8,local_128,0);
  uVar10 = (local_140 - local_e0) - local_100[0];
  bVar5 = true;
  if (local_138 < local_110) {
    bVar5 = false;
    if (uVar10 < local_108) goto LAB_ram_0002f530;
LAB_ram_0002f380:
    if (uVar10 == local_108) goto LAB_ram_0002f388;
LAB_ram_0002f548:
    if (bVar3) goto code_r0x0002f580;
  }
  else {
    if (local_108 <= uVar10) goto LAB_ram_0002f380;
LAB_ram_0002f530:
    bVar3 = false;
    if (uVar10 != local_108) goto LAB_ram_0002f548;
LAB_ram_0002f388:
    if (bVar5) goto code_r0x0002f580;
  }
  uVar7 = local_138 + local_128;
  local_178 = 1;
  uVar8 = uVar10 + uVar9 + (ulonglong)(uVar7 < local_138);
  uVar6 = 1;
  if (uVar7 < local_110) {
    uVar6 = 0;
    if (uVar8 < local_108) goto LAB_ram_0002f810;
LAB_ram_0002f428:
    uVar14 = 1;
    if (uVar8 == local_108) goto LAB_ram_0002f430;
LAB_ram_0002f820:
    uVar6 = uVar14;
    if (uVar10 <= uVar8) goto LAB_ram_0002f838;
LAB_ram_0002f440:
    uVar14 = 1;
    if (uVar8 == uVar10) goto LAB_ram_0002f448;
LAB_ram_0002f848:
    uVar6 = uVar14 | uVar6;
    if (uVar6 == 0) goto LAB_ram_0002f868;
LAB_ram_0002f460:
    local_f0 = local_f0 - 1;
    if (local_f0 != 0xffffffffffffffff) goto LAB_ram_0002f480;
LAB_ram_0002f890:
    lVar4 = 0;
    if (uVar6 == 0) goto LAB_ram_0002f8a8;
LAB_ram_0002f8d0:
    local_178 = 0;
    local_138 = uVar7;
  }
  else {
    if (local_108 <= uVar8) goto LAB_ram_0002f428;
LAB_ram_0002f810:
    uVar14 = 0;
    if (uVar8 != local_108) goto LAB_ram_0002f820;
LAB_ram_0002f430:
    if (uVar8 < uVar10) goto LAB_ram_0002f440;
LAB_ram_0002f838:
    uVar14 = 0;
    if (uVar8 != uVar10) goto LAB_ram_0002f848;
LAB_ram_0002f448:
    uVar6 = uVar7 < local_138 | uVar6;
    if (uVar6 != 0) goto LAB_ram_0002f460;
LAB_ram_0002f868:
    local_f0 = local_f0 - 2;
    if (0xfffffffffffffffd < local_f0) goto LAB_ram_0002f890;
LAB_ram_0002f480:
    lVar4 = 1;
    if (uVar6 != 0) goto LAB_ram_0002f8d0;
LAB_ram_0002f8a8:
    uVar7 = local_128 + uVar7;
    local_138 = uVar7;
    if (local_128 <= uVar7) goto LAB_ram_0002f8d0;
  }
  uVar10 = 0;
  if (uVar6 == 0) {
    uVar10 = uVar9;
  }
  uVar10 = uVar10 + uVar8 + local_178;
  local_e8 = local_e8 + lVar4 + -1;
code_r0x0002f580:
  FUN_ram_00032018(&local_120,local_138 - local_110,
                   (uVar10 - local_108) - (ulonglong)(local_138 < local_110),uVar12);
  param_1[2] = uVar1;
  *param_1 = local_f0;
  param_1[3] = lVar2 + lVar11;
  param_1[1] = local_e8 + lVar13;
  param_1[5] = local_118;
  param_1[4] = local_120;
  return;
}

// Function: FUN_ram_0002fa20
void FUN_ram_0002fa20(undefined8 *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4)

{
  longlong lVar1;
  undefined *local_38;
  undefined8 local_30;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 uStack_18;
  undefined1 local_1;
  
  local_20 = 0;
  local_28 = 0;
  local_30 = 0;
  local_38 = (undefined *)0x0;
  local_1 = 0xff;
  lVar1 = FUN_ram_0002fa90(param_2,param_3,param_4,&local_38,&local_1);
  if (lVar1 == 0) {
    param_1[3] = local_20;
    param_1[2] = local_28;
    param_1[1] = local_30;
    *param_1 = local_38;
    *(undefined1 *)(param_1 + 4) = local_1;
    return;
  }
  local_38 = &DAT_ram_00034710;
  uStack_18 = 0;
  local_30 = 1;
  local_20 = 0;
  local_28 = 8;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&local_38,&DAT_ram_00034720);
}

// Function: FUN_ram_0002fa90
void FUN_ram_0002fa90(void)

{
  longlong lVar1;
  undefined8 *unaff_R6;
  undefined *local_38;
  undefined8 local_30;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_18;
  undefined1 local_1;
  
  lVar1 = FUN_ram_0002fa90();
  if (lVar1 == 0) {
    unaff_R6[3] = local_20;
    unaff_R6[2] = local_28;
    unaff_R6[1] = local_30;
    *unaff_R6 = local_38;
    *(undefined1 *)(unaff_R6 + 4) = local_1;
    return;
  }
  local_38 = &DAT_ram_00034710;
  local_18 = 0;
  local_30 = 1;
  local_20 = 0;
  local_28 = 8;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&local_38,&DAT_ram_00034720);
}

// Function: FUN_ram_0002fb58
void FUN_ram_0002fb58(void)

{
  FUN_ram_0002fb58();
  return;
}

// Function: FUN_ram_0002fb68
void FUN_ram_0002fb68(undefined8 param_1)

{
  undefined *puVar1;
  undefined **ppuVar2;
  undefined8 *puVar3;
  undefined8 **ppuStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  undefined *puStack_18;
  undefined8 *puStack_10;
  undefined8 *puStack_8;
  
  FUN_ram_0002fb70();
  FUN_ram_0002fb70();
  FUN_ram_0002fb78();
  puVar1 = &DAT_ram_00033f40;
  puVar3 = (undefined8 *)0x2b;
  FUN_ram_0002fbd8(&DAT_ram_00033f40,0x2b,param_1);
  puStack_8 = (undefined8 *)CONCAT62(puStack_8._2_6_,1);
  ppuVar2 = &puStack_18;
  puStack_18 = puVar1;
  puStack_10 = puVar3;
  FUN_ram_0002fb68();
  ppuStack_40 = &puStack_10;
  uStack_20 = 0;
  uStack_38 = 1;
  uStack_28 = 0;
  uStack_30 = 8;
  puStack_10 = ppuVar2;
  puStack_8 = puVar3;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&ppuStack_40,param_1);
}

// Function: FUN_ram_0002fb70
void FUN_ram_0002fb70(undefined8 param_1)

{
  undefined *puVar1;
  undefined **ppuVar2;
  undefined8 *puVar3;
  undefined8 **ppuStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  undefined *puStack_18;
  undefined8 *puStack_10;
  undefined8 *puStack_8;
  
  FUN_ram_0002fb70();
  FUN_ram_0002fb78();
  puVar1 = &DAT_ram_00033f40;
  puVar3 = (undefined8 *)0x2b;
  FUN_ram_0002fbd8(&DAT_ram_00033f40,0x2b,param_1);
  puStack_8 = (undefined8 *)CONCAT62(puStack_8._2_6_,1);
  ppuVar2 = &puStack_18;
  puStack_18 = puVar1;
  puStack_10 = puVar3;
  FUN_ram_0002fb68();
  ppuStack_40 = &puStack_10;
  uStack_20 = 0;
  uStack_38 = 1;
  uStack_28 = 0;
  uStack_30 = 8;
  puStack_10 = ppuVar2;
  puStack_8 = puVar3;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&ppuStack_40,param_1);
}

// Function: FUN_ram_0002fb78
void FUN_ram_0002fb78(undefined8 param_1)

{
  undefined *puVar1;
  undefined **ppuVar2;
  undefined8 *puVar3;
  undefined8 **ppuStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  undefined *puStack_18;
  undefined8 *puStack_10;
  undefined8 *puStack_8;
  
  FUN_ram_0002fb78();
  puVar1 = &DAT_ram_00033f40;
  puVar3 = (undefined8 *)0x2b;
  FUN_ram_0002fbd8(&DAT_ram_00033f40,0x2b,param_1);
  puStack_8 = (undefined8 *)CONCAT62(puStack_8._2_6_,1);
  ppuVar2 = &puStack_18;
  puStack_18 = puVar1;
  puStack_10 = puVar3;
  FUN_ram_0002fb68();
  ppuStack_40 = &puStack_10;
  uStack_20 = 0;
  uStack_38 = 1;
  uStack_28 = 0;
  uStack_30 = 8;
  puStack_10 = ppuVar2;
  puStack_8 = puVar3;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&ppuStack_40,param_1);
}

// Function: FUN_ram_0002fb80
void FUN_ram_0002fb80(undefined8 param_1)

{
  undefined *puVar1;
  undefined **ppuVar2;
  undefined8 *puVar3;
  undefined8 **ppuStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  undefined *puStack_18;
  undefined8 *puStack_10;
  undefined8 *puStack_8;
  
  puVar1 = &DAT_ram_00033f40;
  puVar3 = (undefined8 *)0x2b;
  FUN_ram_0002fbd8(&DAT_ram_00033f40,0x2b,param_1);
  puStack_8 = (undefined8 *)CONCAT62(puStack_8._2_6_,1);
  ppuVar2 = &puStack_18;
  puStack_18 = puVar1;
  puStack_10 = puVar3;
  FUN_ram_0002fb68();
  ppuStack_40 = &puStack_10;
  uStack_20 = 0;
  uStack_38 = 1;
  uStack_28 = 0;
  uStack_30 = 8;
  puStack_10 = ppuVar2;
  puStack_8 = puVar3;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&ppuStack_40,param_1);
}

// Function: FUN_ram_0002fba8
void FUN_ram_0002fba8(undefined8 param_1,undefined8 *param_2,undefined8 param_3)

{
  undefined8 *puVar1;
  undefined8 **ppuStack_40;
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined8 uStack_28;
  undefined8 uStack_20;
  undefined8 local_18;
  undefined8 *local_10;
  undefined8 *local_8;
  
  local_8 = (undefined8 *)CONCAT62(local_8._2_6_,1);
  puVar1 = &local_18;
  local_18 = param_1;
  local_10 = param_2;
  FUN_ram_0002fb68();
  ppuStack_40 = &local_10;
  uStack_20 = 0;
  uStack_38 = 1;
  uStack_28 = 0;
  uStack_30 = 8;
  local_10 = puVar1;
  local_8 = param_2;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&ppuStack_40,param_3);
}

// Function: FUN_ram_0002fbd8
void FUN_ram_0002fbd8(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 *local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_10;
  undefined8 local_8;
  
  local_40 = &local_10;
  local_20 = 0;
  local_38 = 1;
  local_28 = 0;
  local_30 = 8;
  local_10 = param_1;
  local_8 = param_2;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&local_40,param_3);
}

// Function: FUN_ram_0002fc40
void FUN_ram_0002fc40(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 local_60;
  undefined8 local_58;
  undefined *local_50;
  undefined8 local_48;
  undefined8 **local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 *local_20;
  undefined1 *local_18;
  undefined8 *local_10;
  undefined1 *local_8;
  
  local_50 = &DAT_ram_00034738;
  local_40 = &local_20;
  local_10 = &local_60;
  local_8 = &LAB_ram_000315a0;
  local_18 = &LAB_ram_000315a0;
  local_20 = &local_58;
  local_30 = 0;
  local_48 = 2;
  local_38 = 2;
  local_60 = param_1;
  local_58 = param_2;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&local_50,param_3);
}

// Function: FUN_ram_0002fd08
void FUN_ram_0002fd08(undefined8 param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
                     undefined8 param_5)

{
  undefined8 local_70;
  undefined8 local_68;
  undefined8 local_60;
  undefined8 local_58;
  undefined *local_50;
  undefined8 local_48;
  undefined8 **local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 *local_20;
  undefined1 *local_18;
  undefined8 *local_10;
  undefined *local_8;
  
  local_50 = &DAT_ram_00034758;
  local_40 = &local_20;
  local_8 = &DAT_ram_00031870;
  local_10 = &local_60;
  local_18 = &LAB_ram_000318a0;
  local_20 = &local_70;
  local_30 = 0;
  local_48 = 2;
  local_38 = 2;
  local_70 = param_1;
  local_68 = param_2;
  local_60 = param_3;
  local_58 = param_4;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&local_50,param_5);
}

// Function: FUN_ram_0002fdf0
/* WARNING: Control flow encountered bad instruction data */

undefined8 FUN_ram_0002fdf0(undefined8 param_1,undefined8 param_2,longlong *param_3)

{
  if (param_3[4] == 0) {
    if (param_3[3] != 0) {
      if (*(longlong *)(*param_3 + 8) != 0) {
                    /* WARNING: Bad instruction - Truncating control flow here */
        halt_baddata();
      }
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    }
  }
  else if (param_3[5] != 0) {
    if (*(longlong *)(*param_3 + 8) != 0) {
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    }
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  }
  if (param_3[1] != 0) {
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  }
  return 0;
}

// Function: FUN_ram_00030230
/* WARNING: Control flow encountered bad instruction data */

undefined8
FUN_ram_00030230(longlong *param_1,longlong param_2,char *param_3,ulonglong param_4,longlong param_5
                )

{
  longlong lVar1;
  char *pcVar2;
  undefined8 uVar3;
  uint uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  undefined1 local_20 [8];
  int local_18;
  undefined1 local_10 [8];
  int local_8;
  
  uVar5 = *(ulonglong *)(param_5 + -0xff8);
  if (param_2 == 0) {
    uVar3 = 0x2d;
    uVar4 = *(uint *)((longlong)param_1 + 0x34);
  }
  else {
    uVar3 = 0x110000;
    uVar4 = *(uint *)((longlong)param_1 + 0x34);
    if ((uVar4 & 1) == 0) goto LAB_ram_000302a0;
    uVar3 = 0x2b;
  }
  uVar5 = uVar5 + 1;
LAB_ram_000302a0:
  if ((uVar4 & 4) == 0) {
    param_3 = (char *)0x0;
  }
  else {
    if (param_4 < 0x20) {
      lVar1 = 0;
      pcVar2 = param_3;
      uVar6 = param_4;
      if (param_4 != 0) {
        do {
          lVar1 = lVar1 + (ulonglong)(-0x41 < *pcVar2);
          uVar6 = uVar6 - 1;
          pcVar2 = pcVar2 + 1;
        } while (uVar6 != 0);
      }
    }
    else {
      lVar1 = FUN_ram_00031050(param_3,param_4);
    }
    uVar5 = lVar1 + uVar5;
  }
  if ((*param_1 == 0) || (uVar6 = param_1[1], uVar6 <= uVar5)) {
    lVar1 = FUN_ram_00030748(param_1);
    if (lVar1 == 0) {
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    }
  }
  else if ((uVar4 & 8) == 0) {
    FUN_ram_00030c80(local_20,param_1,uVar6 - uVar5,1);
    if ((local_18 != 0x110000) &&
       (lVar1 = FUN_ram_00030748(param_1,uVar3,param_3,param_4), lVar1 == 0)) {
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    }
  }
  else {
    *(undefined4 *)(param_1 + 6) = 0x30;
    *(undefined1 *)(param_1 + 7) = 1;
    lVar1 = FUN_ram_00030748(param_1);
    if ((lVar1 == 0) && (FUN_ram_00030c80(local_10,param_1,uVar6 - uVar5,1), local_8 != 0x110000)) {
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    }
  }
  return 1;
}

// Function: FUN_ram_00030748
/* WARNING: Control flow encountered bad instruction data */

undefined8 FUN_ram_00030748(undefined8 param_1,int param_2,longlong param_3)

{
  if (param_2 != 0x110000) {
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  }
  if (param_3 != 0) {
                    /* WARNING: Bad instruction - Truncating control flow here */
    halt_baddata();
  }
  return 0;
}

// Function: FUN_ram_00030808
/* WARNING: Control flow encountered bad instruction data */

undefined8 FUN_ram_00030808(longlong *param_1,byte *param_2,byte *param_3)

{
  byte bVar1;
  byte *pbVar2;
  byte *pbVar3;
  ulonglong uVar4;
  byte *pbVar5;
  byte *pbVar6;
  ulonglong uVar7;
  undefined1 local_10 [8];
  int local_8;
  
  if ((*param_1 == 0) && ((param_1[2] & 1U) == 0)) {
    halt_baddata();
  }
  if ((param_1[2] & 1U) == 0) goto LAB_ram_00030a50;
  pbVar6 = param_2 + (longlong)param_3;
  pbVar5 = (byte *)0x0;
  if (param_1[3] == 0) {
    if (param_2 == pbVar6) goto LAB_ram_00030a50;
  }
  else {
    uVar7 = 0;
    pbVar2 = param_2;
    do {
      if (pbVar2 == pbVar6) goto LAB_ram_00030a50;
      pbVar3 = pbVar2 + 1;
      bVar1 = *pbVar2;
      if ((((char)bVar1 < '\0') && (pbVar3 = pbVar2 + 2, 0xdf < bVar1)) &&
         (pbVar3 = pbVar2 + 3, 0xef < bVar1)) {
        pbVar3 = pbVar2 + 4;
      }
      uVar7 = uVar7 + 1;
      pbVar5 = pbVar3 + ((longlong)pbVar5 - (longlong)pbVar2);
      pbVar2 = pbVar3;
    } while (uVar7 < (ulonglong)param_1[3]);
    if (pbVar3 == pbVar6) goto LAB_ram_00030a50;
  }
  if (pbVar5 == (byte *)0x0) {
LAB_ram_00030a28:
    pbVar6 = param_2;
  }
  else if (pbVar5 < param_3) {
    pbVar6 = (byte *)0x0;
    if (-0x41 < (char)param_2[(longlong)pbVar5]) goto LAB_ram_00030a28;
  }
  else {
    pbVar6 = (byte *)0x0;
    if (pbVar5 == param_3) goto LAB_ram_00030a28;
  }
  if (pbVar6 != (byte *)0x0) {
    param_3 = pbVar5;
    param_2 = pbVar6;
  }
LAB_ram_00030a50:
  if (*param_1 != 0) {
    uVar7 = param_1[1];
    if (param_3 < (byte *)0x20) {
      uVar4 = 0;
      for (; param_3 != (byte *)0x0; param_3 = param_3 + -1) {
        uVar4 = uVar4 + (-0x41 < (char)*param_2);
        param_2 = param_2 + 1;
      }
    }
    else {
      uVar4 = FUN_ram_00031050(param_2,param_3);
    }
    if (uVar4 < uVar7) {
      FUN_ram_00030c80(local_10,param_1,uVar7 - uVar4,0);
      if (local_8 == 0x110000) {
        return 1;
      }
                    /* WARNING: Bad instruction - Truncating control flow here */
      halt_baddata();
    }
  }
                    /* WARNING: Bad instruction - Truncating control flow here */
  halt_baddata();
}

// Function: FUN_ram_00030c80
/* WARNING: Control flow encountered bad instruction data */

void FUN_ram_00030c80(ulonglong *param_1,longlong param_2,ulonglong param_3,char param_4)

{
  byte bVar1;
  ulonglong uVar2;
  ulonglong local_10;
  
  bVar1 = *(byte *)(param_2 + 0x38);
  local_10 = param_3;
  if (bVar1 < 2) {
    if (bVar1 == 0) {
      uVar2 = 0;
      goto LAB_ram_00030d18;
    }
  }
  else {
    if (bVar1 == 2) {
      uVar2 = param_3 >> 1;
      local_10 = param_3 + 1 >> 1;
      goto LAB_ram_00030d18;
    }
    if (param_4 == '\0') {
      uVar2 = 0;
      goto LAB_ram_00030d18;
    }
  }
  local_10 = 0;
  uVar2 = param_3;
LAB_ram_00030d18:
  if (uVar2 == 0) {
    *(undefined4 *)(param_1 + 1) = *(undefined4 *)(param_2 + 0x30);
    *param_1 = local_10;
    return;
  }
                    /* WARNING: Bad instruction - Truncating control flow here */
  halt_baddata();
}

// Function: FUN_ram_00030da0
/* WARNING: Control flow encountered bad instruction data */

void FUN_ram_00030da0(void)

{
                    /* WARNING: Bad instruction - Truncating control flow here */
  halt_baddata();
}

// Function: FUN_ram_00030dd0
void FUN_ram_00030dd0(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  FUN_ram_00030808(param_3,param_1,param_2);
  return;
}

// Function: FUN_ram_00030e00
void FUN_ram_00030e00(undefined8 *param_1,byte param_2,ulonglong param_3,ulonglong param_4)

{
  ulonglong uVar1;
  undefined8 uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  
  uVar3 = 0;
  uVar1 = param_3 + 7 & 0xfffffffffffffff8;
  uVar4 = uVar3;
  if (uVar1 != param_3) {
    uVar1 = uVar1 - param_3;
    if (param_4 <= uVar1) {
      uVar1 = param_4;
    }
    if (uVar1 != 0) {
      do {
        if (*(byte *)(param_3 + uVar3) == param_2) goto LAB_ram_00031018;
        uVar3 = uVar3 + 1;
        uVar4 = uVar1;
      } while (uVar3 < uVar1);
    }
  }
  if (uVar4 <= param_4 - 0x10) {
    do {
      uVar3 = ((ulonglong *)(param_3 + uVar4))[1] ^ (ulonglong)param_2 * 0x101010101010101;
      uVar1 = *(ulonglong *)(param_3 + uVar4) ^ (ulonglong)param_2 * 0x101010101010101;
      if (((0x101010101010100 - uVar1 | uVar1) & (0x101010101010100 - uVar3 | uVar3) &
          0x8080808080808080) != 0x8080808080808080) break;
      uVar4 = uVar4 + 0x10;
    } while (uVar4 <= param_4 - 0x10);
  }
  uVar3 = uVar4;
  if (param_4 == uVar4) {
    uVar2 = 0;
  }
  else {
    uVar1 = 0;
    do {
      if (*(byte *)(param_3 + uVar4 + uVar1) == param_2) {
        uVar3 = uVar4 + uVar1;
LAB_ram_00031018:
        uVar2 = 1;
        goto LAB_ram_00031020;
      }
      uVar1 = uVar1 + 1;
    } while (uVar1 < param_4 - uVar4);
    uVar2 = 0;
  }
LAB_ram_00031020:
  param_1[1] = uVar3;
  *param_1 = uVar2;
  return;
}

// Function: FUN_ram_00031038
longlong FUN_ram_00031038(char *param_1,ulonglong param_2)

{
  longlong lVar1;
  char *pcVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  longlong lVar5;
  ulonglong *puVar6;
  longlong lVar7;
  char *pcVar8;
  ulonglong uVar9;
  
  FUN_ram_000318d0();
  FUN_ram_00031998();
  FUN_ram_00031a60();
  pcVar8 = (char *)((ulonglong)(param_1 + 7) & 0xfffffffffffffff8);
  uVar4 = (longlong)pcVar8 - (longlong)param_1;
  if ((param_2 < uVar4) || (uVar9 = param_2 - uVar4, uVar9 < 8)) {
    lVar1 = 0;
    for (; param_2 != 0; param_2 = param_2 - 1) {
      lVar1 = lVar1 + (ulonglong)(-0x41 < *param_1);
      param_1 = param_1 + 1;
    }
  }
  else {
    uVar3 = uVar9 & 7;
    lVar1 = 0;
    lVar5 = 0;
    if (pcVar8 != param_1) {
      lVar7 = (longlong)param_1 - (longlong)pcVar8;
      pcVar8 = param_1;
      do {
        lVar7 = lVar7 + 1;
        lVar5 = lVar5 + (ulonglong)(-0x41 < *pcVar8);
        pcVar8 = pcVar8 + 1;
      } while (lVar7 != 0);
    }
    if (uVar3 != 0) {
      pcVar8 = param_1 + uVar4 + (uVar9 & 0xfffffffffffffff8);
      lVar1 = 0;
      do {
        lVar1 = lVar1 + (ulonglong)(-0x41 < *pcVar8);
        pcVar8 = pcVar8 + 1;
        uVar3 = uVar3 - 1;
      } while (uVar3 != 0);
    }
    lVar1 = lVar1 + lVar5;
    pcVar8 = param_1 + uVar4;
    uVar4 = uVar9 >> 3;
    do {
      uVar9 = uVar4;
      pcVar2 = pcVar8;
      if (uVar9 == 0) {
        return lVar1;
      }
      uVar3 = uVar9;
      if (0xbf < uVar9) {
        uVar3 = 0xc0;
      }
      uVar4 = 0;
      if (3 < uVar9) {
        pcVar8 = pcVar2;
        do {
          lVar5 = 0;
          do {
            uVar4 = (((*(ulonglong *)(pcVar8 + lVar5) ^ 0xffffffffffffffff) >> 7 |
                     *(ulonglong *)(pcVar8 + lVar5) >> 6) & 0x101010101010101) + uVar4;
            lVar5 = lVar5 + 8;
          } while (lVar5 != 0x20);
          pcVar8 = pcVar8 + 0x20;
        } while (pcVar8 != pcVar2 + (uVar3 * 8 & 0x7e0));
      }
      lVar1 = (((uVar4 >> 8 & 0xff00ff00ff00ff) + (uVar4 & 0xff00ff00ff00ff)) * 0x1000100010001 >>
              0x30) + lVar1;
      pcVar8 = pcVar2 + uVar3 * 8;
      uVar4 = uVar9 - uVar3;
    } while ((uVar3 & 3) == 0);
    if (0xbf < uVar9) {
      uVar9 = 0;
    }
    puVar6 = (ulonglong *)(pcVar2 + (uVar3 & 0xfc) * 8);
    uVar4 = 0;
    lVar5 = (uVar9 & 3) << 3;
    do {
      uVar4 = (((*puVar6 ^ 0xffffffffffffffff) >> 7 | *puVar6 >> 6) & 0x101010101010101) + uVar4;
      puVar6 = puVar6 + 1;
      lVar5 = lVar5 + -8;
    } while (lVar5 != 0);
    lVar1 = (((uVar4 >> 8 & 0xff00ff00ff00ff) + (uVar4 & 0xff00ff00ff00ff)) * 0x1000100010001 >>
            0x30) + lVar1;
  }
  return lVar1;
}

// Function: FUN_ram_00031040
longlong FUN_ram_00031040(char *param_1,ulonglong param_2)

{
  longlong lVar1;
  char *pcVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  longlong lVar5;
  ulonglong *puVar6;
  longlong lVar7;
  char *pcVar8;
  ulonglong uVar9;
  
  FUN_ram_00031998();
  FUN_ram_00031a60();
  pcVar8 = (char *)((ulonglong)(param_1 + 7) & 0xfffffffffffffff8);
  uVar4 = (longlong)pcVar8 - (longlong)param_1;
  if ((param_2 < uVar4) || (uVar9 = param_2 - uVar4, uVar9 < 8)) {
    lVar1 = 0;
    for (; param_2 != 0; param_2 = param_2 - 1) {
      lVar1 = lVar1 + (ulonglong)(-0x41 < *param_1);
      param_1 = param_1 + 1;
    }
  }
  else {
    uVar3 = uVar9 & 7;
    lVar1 = 0;
    lVar5 = 0;
    if (pcVar8 != param_1) {
      lVar7 = (longlong)param_1 - (longlong)pcVar8;
      pcVar8 = param_1;
      do {
        lVar7 = lVar7 + 1;
        lVar5 = lVar5 + (ulonglong)(-0x41 < *pcVar8);
        pcVar8 = pcVar8 + 1;
      } while (lVar7 != 0);
    }
    if (uVar3 != 0) {
      pcVar8 = param_1 + uVar4 + (uVar9 & 0xfffffffffffffff8);
      lVar1 = 0;
      do {
        lVar1 = lVar1 + (ulonglong)(-0x41 < *pcVar8);
        pcVar8 = pcVar8 + 1;
        uVar3 = uVar3 - 1;
      } while (uVar3 != 0);
    }
    lVar1 = lVar1 + lVar5;
    pcVar8 = param_1 + uVar4;
    uVar4 = uVar9 >> 3;
    do {
      uVar9 = uVar4;
      pcVar2 = pcVar8;
      if (uVar9 == 0) {
        return lVar1;
      }
      uVar3 = uVar9;
      if (0xbf < uVar9) {
        uVar3 = 0xc0;
      }
      uVar4 = 0;
      if (3 < uVar9) {
        pcVar8 = pcVar2;
        do {
          lVar5 = 0;
          do {
            uVar4 = (((*(ulonglong *)(pcVar8 + lVar5) ^ 0xffffffffffffffff) >> 7 |
                     *(ulonglong *)(pcVar8 + lVar5) >> 6) & 0x101010101010101) + uVar4;
            lVar5 = lVar5 + 8;
          } while (lVar5 != 0x20);
          pcVar8 = pcVar8 + 0x20;
        } while (pcVar8 != pcVar2 + (uVar3 * 8 & 0x7e0));
      }
      lVar1 = (((uVar4 >> 8 & 0xff00ff00ff00ff) + (uVar4 & 0xff00ff00ff00ff)) * 0x1000100010001 >>
              0x30) + lVar1;
      pcVar8 = pcVar2 + uVar3 * 8;
      uVar4 = uVar9 - uVar3;
    } while ((uVar3 & 3) == 0);
    if (0xbf < uVar9) {
      uVar9 = 0;
    }
    puVar6 = (ulonglong *)(pcVar2 + (uVar3 & 0xfc) * 8);
    uVar4 = 0;
    lVar5 = (uVar9 & 3) << 3;
    do {
      uVar4 = (((*puVar6 ^ 0xffffffffffffffff) >> 7 | *puVar6 >> 6) & 0x101010101010101) + uVar4;
      puVar6 = puVar6 + 1;
      lVar5 = lVar5 + -8;
    } while (lVar5 != 0);
    lVar1 = (((uVar4 >> 8 & 0xff00ff00ff00ff) + (uVar4 & 0xff00ff00ff00ff)) * 0x1000100010001 >>
            0x30) + lVar1;
  }
  return lVar1;
}

// Function: FUN_ram_00031048
longlong FUN_ram_00031048(char *param_1,ulonglong param_2)

{
  longlong lVar1;
  char *pcVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  longlong lVar5;
  ulonglong *puVar6;
  longlong lVar7;
  char *pcVar8;
  ulonglong uVar9;
  
  FUN_ram_00031a60();
  pcVar8 = (char *)((ulonglong)(param_1 + 7) & 0xfffffffffffffff8);
  uVar4 = (longlong)pcVar8 - (longlong)param_1;
  if ((param_2 < uVar4) || (uVar9 = param_2 - uVar4, uVar9 < 8)) {
    lVar1 = 0;
    for (; param_2 != 0; param_2 = param_2 - 1) {
      lVar1 = lVar1 + (ulonglong)(-0x41 < *param_1);
      param_1 = param_1 + 1;
    }
  }
  else {
    uVar3 = uVar9 & 7;
    lVar1 = 0;
    lVar5 = 0;
    if (pcVar8 != param_1) {
      lVar7 = (longlong)param_1 - (longlong)pcVar8;
      pcVar8 = param_1;
      do {
        lVar7 = lVar7 + 1;
        lVar5 = lVar5 + (ulonglong)(-0x41 < *pcVar8);
        pcVar8 = pcVar8 + 1;
      } while (lVar7 != 0);
    }
    if (uVar3 != 0) {
      pcVar8 = param_1 + uVar4 + (uVar9 & 0xfffffffffffffff8);
      lVar1 = 0;
      do {
        lVar1 = lVar1 + (ulonglong)(-0x41 < *pcVar8);
        pcVar8 = pcVar8 + 1;
        uVar3 = uVar3 - 1;
      } while (uVar3 != 0);
    }
    lVar1 = lVar1 + lVar5;
    pcVar8 = param_1 + uVar4;
    uVar4 = uVar9 >> 3;
    do {
      uVar9 = uVar4;
      pcVar2 = pcVar8;
      if (uVar9 == 0) {
        return lVar1;
      }
      uVar3 = uVar9;
      if (0xbf < uVar9) {
        uVar3 = 0xc0;
      }
      uVar4 = 0;
      if (3 < uVar9) {
        pcVar8 = pcVar2;
        do {
          lVar5 = 0;
          do {
            uVar4 = (((*(ulonglong *)(pcVar8 + lVar5) ^ 0xffffffffffffffff) >> 7 |
                     *(ulonglong *)(pcVar8 + lVar5) >> 6) & 0x101010101010101) + uVar4;
            lVar5 = lVar5 + 8;
          } while (lVar5 != 0x20);
          pcVar8 = pcVar8 + 0x20;
        } while (pcVar8 != pcVar2 + (uVar3 * 8 & 0x7e0));
      }
      lVar1 = (((uVar4 >> 8 & 0xff00ff00ff00ff) + (uVar4 & 0xff00ff00ff00ff)) * 0x1000100010001 >>
              0x30) + lVar1;
      pcVar8 = pcVar2 + uVar3 * 8;
      uVar4 = uVar9 - uVar3;
    } while ((uVar3 & 3) == 0);
    if (0xbf < uVar9) {
      uVar9 = 0;
    }
    puVar6 = (ulonglong *)(pcVar2 + (uVar3 & 0xfc) * 8);
    uVar4 = 0;
    lVar5 = (uVar9 & 3) << 3;
    do {
      uVar4 = (((*puVar6 ^ 0xffffffffffffffff) >> 7 | *puVar6 >> 6) & 0x101010101010101) + uVar4;
      puVar6 = puVar6 + 1;
      lVar5 = lVar5 + -8;
    } while (lVar5 != 0);
    lVar1 = (((uVar4 >> 8 & 0xff00ff00ff00ff) + (uVar4 & 0xff00ff00ff00ff)) * 0x1000100010001 >>
            0x30) + lVar1;
  }
  return lVar1;
}

// Function: FUN_ram_00031050
longlong FUN_ram_00031050(char *param_1,ulonglong param_2)

{
  longlong lVar1;
  char *pcVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  longlong lVar5;
  ulonglong *puVar6;
  longlong lVar7;
  char *pcVar8;
  ulonglong uVar9;
  
  pcVar8 = (char *)((ulonglong)(param_1 + 7) & 0xfffffffffffffff8);
  uVar4 = (longlong)pcVar8 - (longlong)param_1;
  if ((param_2 < uVar4) || (uVar9 = param_2 - uVar4, uVar9 < 8)) {
    lVar1 = 0;
    if (param_2 != 0) {
      do {
        lVar1 = lVar1 + (ulonglong)(-0x41 < *param_1);
        param_1 = param_1 + 1;
        param_2 = param_2 - 1;
      } while (param_2 != 0);
    }
  }
  else {
    uVar3 = uVar9 & 7;
    lVar1 = 0;
    lVar5 = 0;
    if (pcVar8 != param_1) {
      lVar7 = (longlong)param_1 - (longlong)pcVar8;
      pcVar8 = param_1;
      do {
        lVar7 = lVar7 + 1;
        lVar5 = lVar5 + (ulonglong)(-0x41 < *pcVar8);
        pcVar8 = pcVar8 + 1;
      } while (lVar7 != 0);
    }
    if (uVar3 != 0) {
      pcVar8 = param_1 + uVar4 + (uVar9 & 0xfffffffffffffff8);
      lVar1 = 0;
      do {
        lVar1 = lVar1 + (ulonglong)(-0x41 < *pcVar8);
        pcVar8 = pcVar8 + 1;
        uVar3 = uVar3 - 1;
      } while (uVar3 != 0);
    }
    lVar1 = lVar1 + lVar5;
    pcVar8 = param_1 + uVar4;
    uVar4 = uVar9 >> 3;
    do {
      uVar9 = uVar4;
      pcVar2 = pcVar8;
      if (uVar9 == 0) {
        return lVar1;
      }
      uVar3 = uVar9;
      if (0xbf < uVar9) {
        uVar3 = 0xc0;
      }
      uVar4 = 0;
      if (3 < uVar9) {
        pcVar8 = pcVar2;
        do {
          lVar5 = 0;
          do {
            uVar4 = (((*(ulonglong *)(pcVar8 + lVar5) ^ 0xffffffffffffffff) >> 7 |
                     *(ulonglong *)(pcVar8 + lVar5) >> 6) & 0x101010101010101) + uVar4;
            lVar5 = lVar5 + 8;
          } while (lVar5 != 0x20);
          pcVar8 = pcVar8 + 0x20;
        } while (pcVar8 != pcVar2 + (uVar3 * 8 & 0x7e0));
      }
      lVar1 = (((uVar4 >> 8 & 0xff00ff00ff00ff) + (uVar4 & 0xff00ff00ff00ff)) * 0x1000100010001 >>
              0x30) + lVar1;
      pcVar8 = pcVar2 + uVar3 * 8;
      uVar4 = uVar9 - uVar3;
    } while ((uVar3 & 3) == 0);
    if (0xbf < uVar9) {
      uVar9 = 0;
    }
    puVar6 = (ulonglong *)(pcVar2 + (uVar3 & 0xfc) * 8);
    uVar4 = 0;
    lVar5 = (uVar9 & 3) << 3;
    do {
      uVar4 = (((*puVar6 ^ 0xffffffffffffffff) >> 7 | *puVar6 >> 6) & 0x101010101010101) + uVar4;
      puVar6 = puVar6 + 1;
      lVar5 = lVar5 + -8;
    } while (lVar5 != 0);
    lVar1 = (((uVar4 >> 8 & 0xff00ff00ff00ff) + (uVar4 & 0xff00ff00ff00ff)) * 0x1000100010001 >>
            0x30) + lVar1;
  }
  return lVar1;
}

// Function: FUN_ram_00031548
void FUN_ram_00031548(undefined8 param_1)

{
  undefined *local_30;
  undefined8 local_28;
  undefined8 local_20;
  undefined8 local_18;
  undefined8 local_10;
  
  local_30 = &DAT_ram_00034778;
  local_10 = 0;
  local_28 = 1;
  local_18 = 0;
  local_20 = 8;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&local_30,param_1);
}

// Function: FUN_ram_000315c8
void FUN_ram_000315c8(ulonglong param_1,undefined8 param_2,undefined8 param_3)

{
  bool bVar1;
  longlong lVar2;
  longlong lVar3;
  ulonglong uVar4;
  undefined2 uStack_16;
  undefined2 auStack_4 [2];
  
  lVar3 = 0x14;
  if (9999 < param_1) {
    uVar4 = param_1;
    lVar2 = 0;
    do {
      lVar3 = lVar2;
      param_1 = uVar4 / 10000;
      *(undefined2 *)((longlong)auStack_4 + lVar3) =
           *(undefined2 *)(&DAT_ram_00033f7f + ((uVar4 % 10000) / 100) * 2);
      *(undefined2 *)((longlong)auStack_4 + lVar3 + 2) =
           *(undefined2 *)(&DAT_ram_00033f7f + ((uVar4 % 10000) % 100) * 2);
      bVar1 = 99999999 < uVar4;
      uVar4 = param_1;
      lVar2 = lVar3 + -4;
    } while (bVar1);
    lVar3 = lVar3 + 0x10;
  }
  if (99 < param_1) {
    uVar4 = (param_1 & 0xffff) / 100;
    *(undefined2 *)((longlong)&uStack_16 + lVar3) =
         *(undefined2 *)(&DAT_ram_00033f7f + ((param_1 + uVar4 * -100) * 2 & 0xfffe));
    param_1 = uVar4;
    lVar3 = lVar3 + -2;
  }
  if (param_1 < 10) {
    *(byte *)((longlong)&uStack_16 + lVar3 + 1) = (byte)param_1 | 0x30;
  }
  else {
    *(undefined2 *)((longlong)&uStack_16 + lVar3) = *(undefined2 *)(&DAT_ram_00033f7f + param_1 * 2)
    ;
  }
  FUN_ram_00030230(param_3,param_2,1,0);
  return;
}

// Function: FUN_ram_000318d0
void FUN_ram_000318d0(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 local_60;
  undefined8 local_58;
  undefined *local_50;
  undefined8 local_48;
  undefined8 **local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 *local_20;
  undefined1 *local_18;
  undefined8 *local_10;
  undefined1 *local_8;
  
  local_50 = &DAT_ram_00034788;
  local_40 = &local_20;
  local_10 = &local_58;
  local_8 = &LAB_ram_000315a0;
  local_18 = &LAB_ram_000315a0;
  local_20 = &local_60;
  local_30 = 0;
  local_48 = 2;
  local_38 = 2;
  local_60 = param_1;
  local_58 = param_2;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&local_50,param_3);
}

// Function: FUN_ram_00031998
void FUN_ram_00031998(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 local_60;
  undefined8 local_58;
  undefined *local_50;
  undefined8 local_48;
  undefined8 **local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 *local_20;
  undefined1 *local_18;
  undefined8 *local_10;
  undefined1 *local_8;
  
  local_50 = &DAT_ram_000347a8;
  local_40 = &local_20;
  local_10 = &local_58;
  local_8 = &LAB_ram_000315a0;
  local_18 = &LAB_ram_000315a0;
  local_20 = &local_60;
  local_30 = 0;
  local_48 = 2;
  local_38 = 2;
  local_60 = param_1;
  local_58 = param_2;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&local_50,param_3);
}

// Function: FUN_ram_00031a60
void FUN_ram_00031a60(undefined8 param_1,undefined8 param_2,undefined8 param_3)

{
  undefined8 local_60;
  undefined8 local_58;
  undefined *local_50;
  undefined8 local_48;
  undefined8 **local_40;
  undefined8 local_38;
  undefined8 local_30;
  undefined8 *local_20;
  undefined1 *local_18;
  undefined8 *local_10;
  undefined1 *local_8;
  
  local_50 = &DAT_ram_000347c8;
  local_40 = &local_20;
  local_10 = &local_58;
  local_8 = &LAB_ram_000315a0;
  local_18 = &LAB_ram_000315a0;
  local_20 = &local_60;
  local_30 = 0;
  local_48 = 2;
  local_38 = 2;
  local_60 = param_1;
  local_58 = param_2;
                    /* WARNING: Subroutine does not return */
  FUN_ram_0002fba8(&local_50,param_3);
}

// Function: FUN_ram_00031b28
undefined8 FUN_ram_00031b28(undefined8 param_1)

{
  FUN_ram_00031b30();
  return param_1;
}

// Function: FUN_ram_00031b30
void FUN_ram_00031b30(void)

{
  FUN_ram_00031b30();
  return;
}

// Function: FUN_ram_00031b48
undefined8 FUN_ram_00031b48(undefined8 param_1)

{
  FUN_ram_00031b50();
  return param_1;
}

// Function: FUN_ram_00031b50
void FUN_ram_00031b50(void)

{
  FUN_ram_00031b50();
  return;
}

// Function: FUN_ram_00031b68
undefined8 FUN_ram_00031b68(undefined8 param_1,undefined1 param_2)

{
  FUN_ram_00031b78(param_1,param_2);
  return param_1;
}

// Function: FUN_ram_00031b78
void FUN_ram_00031b78(void)

{
  FUN_ram_00031b78();
  return;
}

// Function: FUN_ram_00031b90
void FUN_ram_00031b90(undefined8 *param_1)

{
  undefined8 local_10;
  undefined8 local_8;
  
  FUN_ram_00031c98(&local_10);
  param_1[1] = local_8;
  *param_1 = local_10;
  return;
}

// Function: FUN_ram_00031bd8
void FUN_ram_00031bd8(ulonglong *param_1,ulonglong param_2,ulonglong param_3,ulonglong param_4)

{
  if ((param_4 & 0x40) == 0) {
    if ((param_4 & 0xffffffff) != 0) {
      param_3 = param_3 << (param_4 & 0x3f) | param_2 >> (-param_4 & 0x3f);
      param_2 = param_2 << (param_4 & 0x3f);
    }
  }
  else {
    param_3 = param_2 << (param_4 & 0x3f);
    param_2 = 0;
  }
  *param_1 = param_2;
  param_1[1] = param_3;
  return;
}

// Function: FUN_ram_00031c98
void FUN_ram_00031c98(ulonglong *param_1,ulonglong param_2,ulonglong param_3,ulonglong param_4)

{
  if ((param_4 & 0x40) == 0) {
    if ((param_4 & 0xffffffff) != 0) {
      param_2 = param_3 << (-param_4 & 0x3f) | param_2 >> (param_4 & 0x3f);
      param_3 = (longlong)param_3 >> (param_4 & 0x3f);
    }
  }
  else {
    param_2 = (longlong)param_3 >> (param_4 & 0x3f);
    param_3 = (longlong)param_3 >> 0x3f;
  }
  *param_1 = param_2;
  param_1[1] = param_3;
  return;
}

// Function: FUN_ram_00031d60
void FUN_ram_00031d60(ulonglong *param_1,ulonglong param_2,ulonglong param_3,ulonglong param_4)

{
  if ((param_4 & 0x40) == 0) {
    if ((param_4 & 0xffffffff) != 0) {
      param_2 = param_3 << (-param_4 & 0x3f) | param_2 >> (param_4 & 0x3f);
      param_3 = param_3 >> (param_4 & 0x3f);
    }
  }
  else {
    param_2 = param_3 >> (param_4 & 0x3f);
    param_3 = 0;
  }
  *param_1 = param_2;
  param_1[1] = param_3;
  return;
}

// Function: FUN_ram_00031e28
void FUN_ram_00031e28(undefined8 *param_1)

{
  undefined8 local_10;
  undefined8 local_8;
  
  FUN_ram_00031bd8(&local_10);
  param_1[1] = local_8;
  *param_1 = local_10;
  return;
}

// Function: FUN_ram_00031e70
void FUN_ram_00031e70(undefined8 *param_1)

{
  undefined8 local_10;
  undefined8 local_8;
  
  FUN_ram_00031eb8(&local_10);
  param_1[1] = local_8;
  *param_1 = local_10;
  return;
}

// Function: FUN_ram_00031eb8
void FUN_ram_00031eb8(ulonglong *param_1,ulonglong param_2,longlong param_3,ulonglong param_4,
                     longlong param_5)

{
  ulonglong uVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  
  uVar2 = (param_4 & 0xffffffff) * (param_2 & 0xffffffff);
  uVar4 = (param_4 >> 0x20) * (param_2 & 0xffffffff);
  uVar1 = uVar4 + (param_4 & 0xffffffff) * (param_2 >> 0x20);
  uVar3 = uVar2 + (uVar1 << 0x20);
  *param_1 = uVar3;
  param_1[1] = (param_4 >> 0x20) * (param_2 >> 0x20) +
               ((ulonglong)(uVar1 < uVar4) << 0x20 | uVar1 >> 0x20) + (ulonglong)(uVar3 < uVar2) +
               param_5 * param_2 + param_4 * param_3;
  return;
}

// Function: FUN_ram_00032018
void FUN_ram_00032018(undefined8 *param_1)

{
  undefined8 local_10;
  undefined8 local_8;
  
  FUN_ram_00031d60(&local_10);
  param_1[1] = local_8;
  *param_1 = local_10;
  return;
}

// Function: FUN_ram_00032060
void FUN_ram_00032060(ulonglong *param_1,ulonglong param_2,ulonglong param_3,ulonglong param_4,
                     ulonglong param_5)

{
  bool bVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  ulonglong uVar7;
  longlong lVar8;
  ulonglong local_c8;
  longlong local_b0;
  longlong local_a8;
  ulonglong local_a0;
  ulonglong local_98;
  ulonglong local_90 [2];
  ulonglong local_80 [2];
  ulonglong local_70;
  ulonglong local_68;
  ulonglong local_60 [2];
  ulonglong local_50;
  longlong local_48;
  undefined8 local_40;
  undefined8 local_38;
  ulonglong local_30;
  longlong local_28;
  ulonglong local_20 [2];
  ulonglong local_10 [2];
  
  uVar4 = param_5 | param_5 >> 1;
  uVar4 = uVar4 | uVar4 >> 2;
  uVar4 = uVar4 | uVar4 >> 4;
  uVar4 = uVar4 | uVar4 >> 8;
  uVar4 = uVar4 | uVar4 >> 0x10;
  uVar4 = (uVar4 | uVar4 >> 0x20) ^ 0xffffffffffffffff;
  uVar4 = uVar4 - (uVar4 >> 1 & 0x5555555555555555);
  uVar4 = (uVar4 & 0x3333333333333333) + (uVar4 >> 2 & 0x3333333333333333);
  if (param_3 == 0) {
    uVar6 = param_2 | param_2 >> 1;
    uVar6 = uVar6 | uVar6 >> 2;
    uVar6 = uVar6 | uVar6 >> 4;
    uVar6 = uVar6 | uVar6 >> 8;
    uVar6 = uVar6 | uVar6 >> 0x10;
    uVar6 = (uVar6 | uVar6 >> 0x20) ^ 0xffffffffffffffff;
    uVar6 = uVar6 - (uVar6 >> 1 & 0x5555555555555555);
    uVar6 = (uVar6 & 0x3333333333333333) + (uVar6 >> 2 & 0x3333333333333333);
    uVar6 = ((uVar6 + (uVar6 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38) + 0x40;
  }
  else {
    uVar6 = param_3 | param_3 >> 1;
    uVar6 = uVar6 | uVar6 >> 2;
    uVar6 = uVar6 | uVar6 >> 4;
    uVar6 = uVar6 | uVar6 >> 8;
    uVar6 = uVar6 | uVar6 >> 0x10;
    uVar6 = (uVar6 | uVar6 >> 0x20) ^ 0xffffffffffffffff;
    uVar6 = uVar6 - (uVar6 >> 1 & 0x5555555555555555);
    uVar6 = (uVar6 & 0x3333333333333333) + (uVar6 >> 2 & 0x3333333333333333);
    uVar6 = (uVar6 + (uVar6 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38;
  }
  uVar4 = (uVar4 + (uVar4 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38;
  if (param_5 == 0) {
    uVar4 = param_4 | param_4 >> 1;
    uVar4 = uVar4 | uVar4 >> 2;
    uVar4 = uVar4 | uVar4 >> 4;
    uVar4 = uVar4 | uVar4 >> 8;
    uVar4 = uVar4 | uVar4 >> 0x10;
    uVar4 = (uVar4 | uVar4 >> 0x20) ^ 0xffffffffffffffff;
    uVar4 = uVar4 - (uVar4 >> 1 & 0x5555555555555555);
    uVar4 = (uVar4 & 0x3333333333333333) + (uVar4 >> 2 & 0x3333333333333333);
    uVar4 = ((uVar4 + (uVar4 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38) + 0x40;
  }
  if (uVar6 < uVar4) {
    if (uVar6 < 0x40) {
      if (uVar4 < 0x60) {
        if (0x1f < (uVar4 - uVar6 & 0xffffffff)) {
          uVar2 = 0x60 - uVar4;
          FUN_ram_00032018(local_10,param_4,param_5,(longlong)(int)uVar2);
          uVar7 = 0;
          uVar5 = 0;
          local_c8 = param_2;
          do {
            uVar6 = 0x40 - uVar6;
            FUN_ram_00032018(local_20,local_c8,param_3,(longlong)(int)uVar6);
            if ((uVar6 & 0xffffffff) < (uVar2 & 0xffffffff)) {
              uVar4 = param_4;
              FUN_ram_00032018(local_60,param_4,param_5,(longlong)(int)uVar6);
              if (local_60[0] != 0) {
                uVar4 = local_20[0] / local_60[0];
              }
              FUN_ram_00031e70(&local_70);
              bVar1 = local_c8 < local_70;
              if (param_3 != local_68) {
                bVar1 = param_3 < local_68;
              }
              if (bVar1) {
                param_4 = local_c8 + param_4;
                uVar4 = (uVar4 + uVar7) - 1;
                param_3 = ((param_3 + param_5 + (ulonglong)(param_4 < local_c8)) - local_68) -
                          (ulonglong)(param_4 < local_70);
                param_2 = param_4 - local_70;
                uVar5 = uVar5 + (uVar4 < uVar7);
                uVar7 = uVar4;
              }
              else {
                param_3 = (param_3 - local_68) - (ulonglong)(local_c8 < local_70);
                param_2 = local_c8 - local_70;
                uVar5 = uVar5 + (uVar7 + uVar4 < uVar7);
                uVar7 = uVar7 + uVar4;
              }
              goto LAB_ram_000327d0;
            }
            uVar3 = local_20[0] / ((local_10[0] & 0xffffffff) + 1);
            uVar6 = uVar6 - uVar2 & 0x7f;
            FUN_ram_00031e28(&local_30,uVar3,0,uVar6);
            FUN_ram_00031e70(&local_40,uVar3,0,param_4,param_5);
            FUN_ram_00031e28(&local_50,local_40,local_38,uVar6);
            uVar7 = local_30 + uVar7;
            param_3 = (param_3 - local_48) - (ulonglong)(local_c8 < local_50);
            param_2 = local_c8 - local_50;
            if (param_3 == 0) {
              uVar6 = param_2 | param_2 >> 1;
              uVar6 = uVar6 | uVar6 >> 2;
              uVar6 = uVar6 | uVar6 >> 4;
              uVar6 = uVar6 | uVar6 >> 8;
              uVar6 = uVar6 | uVar6 >> 0x10;
              uVar6 = (uVar6 | uVar6 >> 0x20) ^ 0xffffffffffffffff;
              uVar6 = uVar6 - (uVar6 >> 1 & 0x5555555555555555);
              uVar6 = (uVar6 & 0x3333333333333333) + (uVar6 >> 2 & 0x3333333333333333);
              uVar6 = ((uVar6 + (uVar6 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38) +
                      0x40;
            }
            else {
              uVar6 = param_3 | param_3 >> 1;
              uVar6 = uVar6 | uVar6 >> 2;
              uVar6 = uVar6 | uVar6 >> 4;
              uVar6 = uVar6 | uVar6 >> 8;
              uVar6 = uVar6 | uVar6 >> 0x10;
              uVar6 = (uVar6 | uVar6 >> 0x20) ^ 0xffffffffffffffff;
              uVar6 = uVar6 - (uVar6 >> 1 & 0x5555555555555555);
              uVar6 = (uVar6 & 0x3333333333333333) + (uVar6 >> 2 & 0x3333333333333333);
              uVar6 = (uVar6 + (uVar6 >> 4) & 0xf0f0f0f0f0f0f0f) * 0x101010101010101 >> 0x38;
            }
            uVar5 = local_28 + uVar5 + (ulonglong)(uVar7 < local_30);
            if (uVar4 <= uVar6) {
              bVar1 = param_2 < param_4;
              if (param_3 != param_5) {
                bVar1 = param_3 < param_5;
              }
              if (!bVar1) {
                uVar7 = uVar7 + 1;
                param_3 = (param_3 - param_5) - (ulonglong)(param_2 < param_4);
                uVar5 = uVar5 + (uVar7 == 0);
                param_2 = param_2 - param_4;
              }
              goto LAB_ram_000327d0;
            }
            local_c8 = param_2;
          } while (uVar6 < 0x40);
          if (param_4 != 0) {
            uVar6 = param_2 / param_4;
          }
          param_3 = 0;
          param_2 = param_2 % param_4;
          uVar5 = uVar5 + (uVar7 + uVar6 < uVar7);
          uVar7 = uVar7 + uVar6;
          goto LAB_ram_000327d0;
        }
        lVar8 = (longlong)(0x40 - (int)uVar6);
        FUN_ram_00032018(local_80,param_4,param_5,lVar8);
        FUN_ram_00032018(local_90,param_2,param_3,lVar8);
        uVar7 = local_90[0] / local_80[0];
        FUN_ram_00031e70(&local_a0,param_4,0,uVar7,0);
        FUN_ram_00031e70(&local_b0,param_5,0,uVar7,0);
        uVar4 = local_98 + local_b0;
        if (local_a8 + (ulonglong)(uVar4 < local_98) == 0) {
          bVar1 = param_2 < local_a0;
          if (param_3 != uVar4) {
            bVar1 = param_3 < uVar4;
          }
          if (!bVar1) {
            param_3 = (param_3 - uVar4) - (ulonglong)(param_2 < local_a0);
            param_2 = param_2 - local_a0;
            goto LAB_ram_000327c8;
          }
        }
        param_2 = param_4 + param_2;
        param_3 = ((param_5 + param_3 + (ulonglong)(param_2 < param_4)) - uVar4) -
                  (ulonglong)(param_2 < local_a0);
        param_2 = param_2 - local_a0;
        uVar5 = 0;
        uVar7 = uVar7 - 1;
        goto LAB_ram_000327d0;
      }
      uVar6 = param_4 & 0xffffffff;
      uVar4 = (param_3 - (param_3 / uVar6) * param_4 << 0x20 | param_2 >> 0x20) / uVar6;
      uVar5 = uVar4 >> 0x20 | param_3 / uVar6;
      param_2 = (param_2 >> 0x20) - uVar4 * param_4 << 0x20 | param_2 & 0xffffffff;
      uVar2 = param_2 / uVar6;
      uVar7 = uVar4 << 0x20 | uVar2;
      param_2 = param_2 - uVar2 * uVar6;
    }
    else {
      uVar7 = param_2 / param_4;
      param_2 = param_2 - uVar7 * param_4;
      uVar5 = 0;
    }
    param_3 = 0;
  }
  else {
    uVar7 = 0;
    bVar1 = param_2 < param_4;
    if (param_3 != param_5) {
      bVar1 = param_3 < param_5;
    }
    if (!bVar1) {
      uVar7 = 1;
      param_3 = (param_3 - param_5) - (ulonglong)(param_2 < param_4);
      param_2 = param_2 - param_4;
    }
LAB_ram_000327c8:
    uVar5 = 0;
  }
LAB_ram_000327d0:
  param_1[2] = param_2;
  *param_1 = uVar7;
  param_1[3] = param_3;
  param_1[1] = uVar5;
  return;
}

// Function: FUN_ram_000334a8
void FUN_ram_000334a8(undefined8 *param_1)

{
  undefined8 local_20;
  undefined8 local_18;
  
  FUN_ram_00032060(&local_20);
  param_1[1] = local_18;
  *param_1 = local_20;
  return;
}

// Function: <EXTERNAL>::abort
/* WARNING: Control flow encountered bad instruction data */
/* WARNING: Unknown calling convention -- yet parameter storage is locked */

void abort(void)

{
                    /* WARNING: Bad instruction - Truncating control flow here */
  halt_baddata();
}

