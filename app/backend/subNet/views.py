from django.shortcuts import render
from .models import Net
from django.contrib.auth.models import User

from rest_framework import viewsets
from rest_framework.response import Response

from .serializers import NetSerializer, UserSerializer

import rsnet
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


class PrefixPost(viewsets.ViewSet):
    def create(self, request, pk=0):
        base_address = request.data.get("baseAddress")
        mask = request.data.get("mask")
        base_address = rsnet.Address(
            base_address[0], base_address[1], base_address[2], base_address[3])
        mask = rsnet.Address(
            mask[0], mask[1], mask[2], mask[3])
        base_net = rsnet.Net(base_address, mask)
        nets = rsnet.scaffold_prefixes(base_net, request.data.get("prefixes"))
        child = Net(
            network_address=".".join(
                str(netbyte) for netbyte in base_net.get_network_address().as_vec()),
            mask=".".join(
                str(netbyte) for netbyte in base_net.get_mask().as_vec()),
            broadcast=".".join(
                str(netbyte) for netbyte in base_net.get_broadcast().as_vec()),
            wildcard=".".join(
                str(netbyte) for netbyte in base_net.get_wildcard().as_vec()),
            prefix=base_net.get_prefix(),
        )
        child.save()
        response = {
            0: {
                "networkAddress": child.network_address,
                "broadcast": child.broadcast,
                "mask": child.mask,
                "wildcard": child.wildcard,
                "prefix": child.prefix
            }
        }
        iterator = 1
        for net in nets:
            child = Net(
                parent=child,
                network_address=".".join(
                    str(netbyte) for netbyte in net.get_network_address().as_vec()),
                mask=".".join(
                    str(netbyte) for netbyte in net.get_mask().as_vec()),
                broadcast=".".join(
                    str(netbyte) for netbyte in net.get_broadcast().as_vec()),
                wildcard=".".join(
                    str(netbyte) for netbyte in net.get_wildcard().as_vec()),
                prefix=net.get_prefix(),
            )
            response[iterator] = {
                "networkAddress": child.network_address,
                "broadcast": child.broadcast,
                "mask": child.mask,
                "wildcard": child.wildcard,
                "prefix": child.prefix
            }
            child.save()
            iterator += 1
        return Response(response)
