#!/bin/bash
# bbcpr - Uninstall Script

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

BINARY_NAME="bbcpr"
INSTALL_DIRS=("/usr/local/bin" "/usr/bin" "$HOME/.local/bin" "$HOME/bin")

echo -e "${YELLOW}Uninstalling bbcpr...${NC}"
echo ""

FOUND=0
for dir in "${INSTALL_DIRS[@]}"; do
    if [ -f "$dir/$BINARY_NAME" ]; then
        echo -e "${YELLOW}Found bbcpr in: $dir${NC}"
        FOUND=1
        
        if [ -w "$dir" ]; then
            rm "$dir/$BINARY_NAME"
        else
            sudo rm "$dir/$BINARY_NAME"
        fi
        
        echo -e "${GREEN}✅ Removed bbcpr from $dir${NC}"
    fi
done

if [ $FOUND -eq 0 ]; then
    echo -e "${YELLOW}bbcpr not found in standard locations${NC}"
    
    # Check if it's in PATH
    if command -v bbcpr &> /dev/null; then
        LOCATION=$(which bbcpr)
        echo -e "${YELLOW}Found bbcpr at: $LOCATION${NC}"
        echo -e "${RED}Please remove manually: sudo rm $LOCATION${NC}"
    else
        echo -e "${GREEN}bbcpr is not installed${NC}"
    fi
else
    echo ""
    echo -e "${GREEN}✅ bbcpr has been uninstalled${NC}"
fi