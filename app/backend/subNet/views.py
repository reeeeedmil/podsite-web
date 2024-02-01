from django.shortcuts import render
from .models import Net
from django.contrib.auth.models import User

from rest_framework import viewsets
from rest_framework.response import Response

from .serializers import NetSerializer, UserSerializer

import subNet.calculations as calculations
import rsnet as net
import subNet.messages as messages
# Create your views here.


class NetViewerViewSet(viewsets.ViewSet):
    def retrieve(self, request, pk=0):
        queryset = Net.objects.get(pk=pk)
        serializer = NetSerializer(queryset)
        return Response(serializer.data)


class NetViewSet(viewsets.ViewSet):
    def retrieve(self, request, pk=0):
        parent = Net.objects.get(pk=pk)
        children = Net.objects.filter(parent=pk)

        serialized_parent = NetSerializer(parent)
        serialized_children = NetSerializer(children, many=True)

        response = {
            'parent': serialized_parent.data,
            'children': serialized_children.data,
        }
        return Response(response)

    def create(self, request):
        def handle_error(errors):
            return Response({"ERRORS": errors})

        network_address = request.data.get("network_address")
        mask = request.data.get("mask")
        prefix_over_mask = request.data.get("prefix_over_mask")
        error = []

        match prefix_over_mask:
            case ('True' | 'true' | 'TRUE'):
                prefix_over_mask = True
            case ('False' | 'false' | 'FALSE'):
                prefix_over_mask = False
            case _:
                error.append(messages.errors.get("POM_NOT_BOOL"))
        try:
            network_address = network_address.split('.')
            network_address = [int(byte) for byte in network_address]
            if len(network_address) != 4:
                error.append(messages.errors.get("NET_ADD_NOT_4"))
            for byte in network_address:
                if byte > 255 or byte < 0:
                    error.append(messages.errors.get("NET_OUT_OF_RANGE"))
                    handle_error(error)

        except Exception:
            error.append(messages.errors.get("NET_UNEXPECTED_CHARS"))
            handle_error(error)

        if prefix_over_mask is False:
            try:
                mask = mask.split('.')
                mask = [int(byte) for byte in mask]
                prefix = calculations.prefix_from_mask(mask)
                if len(mask) != 4:
                    return Response({"ERROR": "Mask is not 4 numbers at POST"})
                for byte in mask:
                    if byte > 255 or byte < 0:
                        return Response({"ERROR": "Mask has a number over 255 or under 0 at POST."})
            except ValueError:
                return Response({"ERROR": "Mask includes unexpected characters at POST."})
        else:
            prefix = int(mask)
            mask = calculations.mask_from_prefix(int(prefix))

        wildcard = calculations.wildcard_calculation(mask)
        broadcast = calculations.broadcast_calculation(network_address, mask)

        network = calculations.IPv4Network()

        network.update_network_address(network_address)
        network.update_mask(mask)
        network.update_prefix(prefix)
        network.update_wildcard(wildcard)
        network.update_broadcast(broadcast)
        if len(error) == 0:
            return Response({"Network address": network.network_address,
                            "mask": network.mask,
                             "wildcard": network.wildcard,
                             "broadcast": network.broadcast,
                             "prefix": network.prefix,
                             })
